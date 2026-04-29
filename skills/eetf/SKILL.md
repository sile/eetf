---
name: eetf
description: Encode and decode Erlang External Term Format (ETF) in Rust using the `eetf` crate. Use when working with binary terms produced by `term_to_binary/1` in Erlang/Elixir, building Rust services that interop with the BEAM (e.g. distribution protocol, ports, NIFs, ErlPort, Mnesia dumps), parsing `.beam` artifacts, or decoding/encoding any payload that starts with the version byte `131` (`0x83`).
license: MIT
metadata:
  repository: https://github.com/sile/eetf
  crate: eetf
  rust-edition: "2024"
---

# eetf — Erlang External Term Format for Rust

`eetf` is a pure-Rust implementation of Erlang's [External Term Format][etf-spec]. It encodes and decodes the same binary payloads that `erlang:term_to_binary/1` and `:erlang.term_to_binary/1` (Elixir) produce.

[etf-spec]: https://erlang.org/doc/apps/erts/erl_ext_dist.html

Use this skill when the user is:

- Decoding bytes that start with `131` (`0x83`, the ETF version tag) into a Rust value.
- Building a payload to hand back to Erlang/Elixir (Erlang distribution, Erlang ports, ErlPort, Mnesia, `:erlang.binary_to_term/1`).
- Pattern-matching against decoded terms (e.g. `{ok, Value}`-shaped tuples).
- Converting between BEAM types (atoms, binaries, pids, refs, big integers, maps, improper lists) and Rust types.

## Setup

Add to `Cargo.toml`:

```toml
[dependencies]
eetf = "0.11"
```

The crate uses Rust edition 2024. It pulls in `num-bigint`, `num-traits`, and `noflate` (zlib decompression for the `COMPRESSED` ETF tag). No `unsafe`, no async, no feature flags.

## Core API at a glance

Everything lives on `Term`. Decoding takes any `io::Read`; encoding takes any `io::Write`.

```rust
use eetf::Term;
use std::io::Cursor;

// Decode
let term = Term::decode(Cursor::new(&bytes))?;

// Encode
let mut buf = Vec::new();
term.encode(&mut buf)?;
```

`Term` is an enum over every ETF type:

| Variant       | Erlang/Elixir analogue                | Notes                                                          |
| ------------- | ------------------------------------- | -------------------------------------------------------------- |
| `Atom`        | `foo`, `'with spaces'`, `:foo`        | UTF-8. Encoded as `SMALL_ATOM_UTF8_EXT` when ≤255 bytes.       |
| `FixInteger`  | small int                             | `i32`.                                                         |
| `BigInteger`  | big int                               | Wraps `num_bigint::BigInt`.                                    |
| `Float`       | float                                 | **Constructed via `TryFrom<f64>`/`TryFrom<f32>`** — rejects NaN/±∞ with `DecodeError::NonFiniteFloat`. |
| `Pid`         | `<0.123.0>`                           | `node`, `id`, `serial`, `creation`.                            |
| `Port`        | `#Port<...>`                          | `node`, `id`, `creation`.                                      |
| `Reference`   | `#Ref<...>`                           | Boxed inside `Term`. `id` is `Vec<u32>`.                       |
| `ExternalFun` | `fun mod:fun/arity`                   | Boxed.                                                         |
| `InternalFun` | `fun () -> ... end`                   | Boxed. `Old` and `New` variants.                               |
| `Binary`      | `<<_/binary>>`                        | `Vec<u8>`.                                                     |
| `BitBinary`   | `<<_:N>>` (non-byte-aligned)          | `bytes` + `tail_bits_size`.                                    |
| `ByteList`    | `STRING_EXT` (list of u8)             | **Distinct from `List`** — see gotcha below.                   |
| `List`        | `[...]`                               | `Vec<Term>`.                                                   |
| `ImproperList`| `[a, b \| c]`                         | `elements` + boxed `last`.                                     |
| `Tuple`       | `{...}`                               | `Vec<Term>`.                                                   |
| `Map`         | `#{...}`                              | `HashMap<Term, Term>` — **insertion order is not preserved**.  |

Every concrete type implements `Display`, `Debug`, `PartialEq`, `Hash` (except `Float` needs care), and `Clone`. Most also impl `From<...>` from common Rust types — `Atom::from("foo")`, `FixInteger::from(42i32)`, `Binary::from(vec![1,2,3])`, `Tuple::from(vec![Term::from(...)])`, etc.

Errors are `DecodeError` and `EncodeError` (both `std::error::Error`). Their type aliases are `DecodeResult = Result<Term, DecodeError>` and `EncodeResult = Result<(), EncodeError>`.

## Decode example

```rust
use eetf::{Term, Atom};
use std::io::Cursor;

let bytes = vec![131, 119, 3, 102, 111, 111]; // term_to_binary(foo)
let term = Term::decode(Cursor::new(&bytes)).unwrap();
assert_eq!(term, Term::from(Atom::from("foo")));
```

Reading from a file:

```rust
use std::fs::File;
let term = Term::decode(File::open("snapshot.etf")?)?;
```

The decoder requires the leading version byte `131`. Strip it yourself if you receive the *content* without the version prefix (rare).

## Encode example

```rust
use eetf::{Term, Tuple, Atom, FixInteger};

let term = Term::from(Tuple::from(vec![
    Term::from(Atom::from("ok")),
    Term::from(FixInteger::from(42)),
]));

let mut buf = Vec::new();
term.encode(&mut buf).unwrap();
// buf[0] == 131
```

Writing to a `TcpStream`, `BufWriter`, etc. is identical — anything implementing `io::Write`.

## Pattern matching

The `pattern` module and `Term::as_match` give a typed shape-matching API. Useful for picking apart `{ok, Value}` / `{error, Reason}` style replies without writing nested `if let` chains.

```rust
use eetf::{Term, Tuple, Atom, FixInteger};
use eetf::pattern::any;

let t = Term::from(Tuple::from(vec![
    Term::from(Atom::from("ok")),
    Term::from(FixInteger::from(7)),
]));

// Match against (literal-atom, any-FixInteger)
let (_ok, n) = t.as_match(("ok", any::<FixInteger>())).unwrap();
assert_eq!(n.value, 7);
```

Useful pattern building blocks (in `eetf::pattern`):

- Tuple-shaped patterns via Rust tuples — `("foo", any::<Atom>())` matches a 2-tuple of literal atom `foo` and any atom.
- `any::<T>()` — accept any value of type `T` (`Atom`, `FixInteger`, `BigInteger`, `Float`, `Binary`, `Tuple`, `List`, `Map`, etc.).
- `Union2` … `Union6` — alternation between several pattern shapes; returns a tagged enum.
- `U8`, `I32`, etc. — integer-range patterns.
- `Unmatch` carries `input`, `pattern`, and an optional `cause` chain so failures are debuggable.

For ad-hoc extraction, `convert::TryAsRef` is the lower-level escape hatch:

```rust
use eetf::convert::TryAsRef;
use eetf::{Term, Atom};

let t: Term = /* ... */;
if let Some(atom) = <Term as TryAsRef<Atom>>::try_as_ref(&t) {
    println!("{}", atom.name);
}
```

`Term` also supports `TryInto<T>` for each variant — `let atom: Atom = term.try_into()?;` (errors return the original term back, not a typed error).

## Common gotchas

### 1. `ByteList` vs `List<FixInteger>`

Erlang represents short lists of bytes (`[1,2,3]` where every element fits in `u8`) using `STRING_EXT`. `eetf` decodes those into `Term::ByteList(ByteList { bytes: Vec<u8> })`, **not** `Term::List`. If you blindly match on `Term::List`, you'll miss byte-list payloads.

To normalize, convert: `List::from(byte_list)` produces a `List` of `FixInteger`s. To accept either, match both variants or pattern with a union.

`String::from(...) -> Term::ByteList(...)` (i.e. `Term::from("hello")` becomes `ByteList`, not a binary). If you want a `<<"hello">>` Erlang binary, use `Term::from(Binary::from(b"hello".as_slice()))`.

### 2. Atoms are NOT strings

Erlang atoms (`foo`) and binaries (`<<"foo">>`) are distinct types. Sending `Term::from(Atom::from("foo"))` is **not** the same as `Term::from(Binary::from(b"foo".as_slice()))` on the receiving Erlang side. When in doubt, mirror the producer side.

### 3. `Float` rejects non-finite values

`Float` only stores finite numbers. Both `TryFrom<f32>` and `TryFrom<f64>` return `Err(DecodeError::NonFiniteFloat)` for NaN/±∞ — there is no `From<f64>` for `Float`. ETF itself has no representation for non-finite floats, so this matches the wire format.

```rust
use eetf::Float;
let f = Float::try_from(1.5_f64)?; // OK
assert!(Float::try_from(f64::NAN).is_err());
```

### 4. Atom name length cap on encode

`EncodeError::TooLongAtomName` fires if an atom exceeds 255 UTF-8 bytes. Erlang atoms have the same limit, so this is a real wire-format constraint, not a library quirk.

### 5. `Map` does not preserve insertion order

Internally `Map` wraps `HashMap<Term, Term>`. Round-tripping a map will re-encode in arbitrary key order. Tests that assert on raw byte equality of encoded maps are flaky; assert on the decoded `Term` instead.

### 6. Compressed payloads are handled transparently

ETF can wrap a payload with the `COMPRESSED` (zlib) tag. `Term::decode` decompresses automatically via `noflate`; you don't need to detect it yourself. `Term::encode` does **not** compress — if you need a compressed wire format, you must wrap externally.

### 7. PID/Port/Reference equality includes `creation`

Two PIDs from different node restarts have different `creation` values and won't compare equal. If you want to ignore `creation` (e.g. to dedupe across restarts), strip it before comparing.

### 8. The version byte is required

Decoding bytes that don't start with `131` yields `DecodeError::UnsupportedVersion { version }`. If you receive an Erlang distribution payload that has been pre-stripped, prepend `131` before calling `decode`.

## Constructing common shapes (cheat sheet)

```rust
use eetf::{Term, Atom, FixInteger, Tuple, List, Binary, Map};

// {ok, 42}
Term::from(Tuple::from(vec![
    Term::from(Atom::from("ok")),
    Term::from(FixInteger::from(42)),
]));

// [1, 2, 3] as a proper list (NOT ByteList)
Term::from(List::from(vec![
    Term::from(FixInteger::from(1)),
    Term::from(FixInteger::from(2)),
    Term::from(FixInteger::from(3)),
]));

// <<"hello">>
Term::from(Binary::from(b"hello".as_slice()));

// #{name => "alice", age => 30}
Term::from(Map::from([
    (Term::from(Atom::from("name")), Term::from(Binary::from(b"alice".as_slice()))),
    (Term::from(Atom::from("age")),  Term::from(FixInteger::from(30))),
]));
```

## Error handling

```rust
use eetf::{Term, DecodeError};
use std::io::Cursor;

match Term::decode(Cursor::new(&bytes)) {
    Ok(t) => { /* ... */ }
    Err(DecodeError::UnsupportedVersion { version }) => { /* not 131 */ }
    Err(DecodeError::UnknownTag { tag }) => { /* corrupt or future tag */ }
    Err(DecodeError::UnexpectedType { value, expected }) => { /* shape mismatch */ }
    Err(DecodeError::OutOfRange { value, range }) => { /* bigint exceeded i32 etc. */ }
    Err(DecodeError::NonFiniteFloat) => { /* unreachable for valid ETF, but possible from TryFrom */ }
    Err(DecodeError::Io(e)) => { /* underlying reader error */ }
}
```

`EncodeError` is much smaller: `Io`, `TooLongAtomName`, `TooLargeInteger`, `TooLargeReferenceId`. The last three correspond to wire-format limits — there is no recovery besides changing the input.

## Interop notes

- **Producing input for `:erlang.binary_to_term/1` / `binary_to_term/1`**: Encode with `eetf`, send the raw bytes (including the leading `131`). For `:erlang.binary_to_term(bin, [:safe])`, ensure all atoms in the payload already exist in the receiving VM, or the call fails.
- **Distribution protocol payloads**: ETF is the carrier, but distribution adds its own framing on top. `eetf` only handles the term encoding, not the framing.
- **`.beam` files**: BEAM chunks are not raw ETF; they have their own container format. Use a dedicated BEAM parser for those (eetf only decodes the embedded ETF chunks if you locate them yourself).

## Verifying your work

The repo's own test suite is a good reference for byte-level expectations: see `tests/lib.rs` for round-trip assertions per term type. When debugging an unexpected encoding, compare bytes against `:erlang.term_to_binary(value)` from an iex/erl shell.

```sh
# Quick sanity check from an Erlang shell:
# erl -noshell -eval 'io:format("~w~n", [binary_to_list(term_to_binary({ok,42}))]), halt().'
```
