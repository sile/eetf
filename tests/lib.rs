extern crate eetf;
extern crate num;

use std::io::Cursor;
use eetf::*;

#[test]
fn atom_test() {
    // Display
    assert_eq!("'foo'", Atom::from("foo").to_string());
    assert_eq!(r#"'fo\'o'"#, Atom::from(r#"fo'o"#).to_string());
    assert_eq!(r#"'fo\\o'"#, Atom::from(r#"fo\o"#).to_string());

    // Decode
    assert_eq!(Some(&Atom::from("foo")),
               decode(&[131, 100, 0, 3, 102, 111, 111]).as_atom()); // ATOM_EXT
    assert_eq!(Some(&Atom::from("foo")),
               decode(&[131, 115, 3, 102, 111, 111]).as_atom()); // SMALL_ATOM_EXT
    assert_eq!(Some(&Atom::from("foo")),
               decode(&[131, 118, 0, 3, 102, 111, 111]).as_atom()); // ATOM_UTF8_EXT
    assert_eq!(Some(&Atom::from("foo")),
               decode(&[131, 119, 3, 102, 111, 111]).as_atom()); // SMALL_ATOM_UTF8_EXT

    // Encode
    assert_eq!(vec![131, 100, 0, 3, 102, 111, 111],
               encode(Term::from(Atom::from("foo"))));
}

#[test]
fn integer_test() {
    // Display
    assert_eq!("123", FixInteger::from(123).to_string());
    assert_eq!("123", BigInteger::from(123).to_string());
    assert_eq!("-123", FixInteger::from(-123).to_string());
    assert_eq!("-123", BigInteger::from(-123).to_string());

    // Decode
    assert_eq!(Some(&FixInteger::from(10)),
               decode(&[131, 97, 10]).as_fix_integer()); // SMALL_INTEGER_EXT
    assert_eq!(Some(&FixInteger::from(1000)),
               decode(&[131, 98, 0, 0, 3, 232]).as_fix_integer()); // INTEGER_EXT
    assert_eq!(Some(&FixInteger::from(-1000)),
               decode(&[131, 98, 255, 255, 252, 24]).as_fix_integer()); // INTEGER_EXT
    assert_eq!(Some(&BigInteger::from(0)),
               decode(&[131, 110, 1, 0, 0]).as_big_integer()); // SMALL_BIG_EXT
    assert_eq!(Some(&BigInteger::from(513)),
               decode(&[131, 110, 2, 0, 1, 2]).as_big_integer()); // SMALL_BIG_EXT
    assert_eq!(Some(&BigInteger::from(-513)),
               decode(&[131, 110, 2, 1, 1, 2]).as_big_integer()); // SMALL_BIG_EXT
    assert_eq!(Some(&BigInteger::from(513)),
               decode(&[131, 111, 0, 0, 0, 2, 0, 1, 2]).as_big_integer()); // LARGE_BIG_EXT

    // Encode
    assert_eq!(vec![131, 97, 0], encode(Term::from(FixInteger::from(0))));
    assert_eq!(vec![131, 98, 255, 255, 255, 255],
               encode(Term::from(FixInteger::from(-1))));
    assert_eq!(vec![131, 98, 0, 0, 3, 232],
               encode(Term::from(FixInteger::from(1000))));
    assert_eq!(vec![131, 110, 5, 0, 0, 228, 11, 84, 2],
               encode(Term::from(FixInteger::from(10000000000))));
    assert_eq!(vec![131, 110, 1, 0, 0],
               encode(Term::from(BigInteger::from(0))));
    assert_eq!(vec![131, 110, 1, 1, 10],
               encode(Term::from(BigInteger::from(-10))));
    assert_eq!(vec![131, 110, 5, 0, 0, 228, 11, 84, 2],
               encode(Term::from(BigInteger::from(10000000000))));
}

#[test]
fn float_test() {
    // Display
    assert_eq!("123", Float::from(123.0).to_string());
    assert_eq!("123.4", Float::from(123.4).to_string());
    assert_eq!("-123.4", Float::from(-123.4).to_string());

    // Decode
    assert_eq!(Some(&Float::from(123.456)),
               decode(&[131, 70, 64, 94, 221, 47, 26, 159, 190, 119]).as_float()); // NEW_FLOAT_EXT
    assert_eq!(Some(&Float::from(-123.456)),
               decode(&[131, 70, 192, 94, 221, 47, 26, 159, 190, 119]).as_float()); // NEW_FLOAT_EXT
    // Encode
    assert_eq!(vec![131, 70, 64, 94, 221, 47, 26, 159, 190, 119],
               encode(Term::from(Float::from(123.456))));
}

fn encode(term: Term) -> Vec<u8> {
    let mut buf = Vec::new();
    term.encode(&mut buf).unwrap();
    buf
}

fn decode(bytes: &[u8]) -> Term {
    Term::decode(Cursor::new(bytes)).unwrap()
}
