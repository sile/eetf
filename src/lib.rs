//! Library for encoding/decoding Erlang External Term Format.
//!
//! # Examples
//!
//! Decodes an atom:
//!
//! ```
//! use std::io::Cursor;
//! use eetf::{Term, Atom};
//!
//! let bytes = vec![131, 100, 0, 3, 102, 111, 111];
//! let term = Term::decode(Cursor::new(&bytes)).unwrap();
//! assert_eq!(term, Term::from(Atom::from("foo")));
//! ```
//!
//! Encodes an atom:
//!
//! ```
//! use eetf::{Term, Atom};
//!
//! let mut buf = Vec::new();
//! let term = Term::from(Atom::from("foo"));
//! term.encode(&mut buf).unwrap();
//! assert_eq!(vec![131, 100, 0, 3, 102, 111, 111], buf);
//! ```
//!
//! # Reference
//!
//! - [Erlang External Term Format](http://erlang.org/doc/apps/erts/erl_ext_dist.html)
//!
use num::bigint::BigInt;
use std::fmt;
use std::io;

mod codec;
pub mod convert;
pub mod pattern;

pub use crate::codec::DecodeError;
pub use crate::codec::DecodeResult;
pub use crate::codec::EncodeError;
pub use crate::codec::EncodeResult;

/// Term.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Term {
    Atom(Atom),
    FixInteger(FixInteger),
    BigInteger(BigInteger),
    Float(Float),
    Pid(Pid),
    Port(Port),
    Reference(Reference),
    ExternalFun(ExternalFun),
    InternalFun(InternalFun),
    Binary(Binary),
    BitBinary(BitBinary),
    List(List),
    ImproperList(ImproperList),
    Tuple(Tuple),
    Map(Map),
}
impl Term {
    /// Decodes a term.
    pub fn decode<R: io::Read>(reader: R) -> DecodeResult {
        codec::Decoder::new(reader).decode()
    }

    /// Encodes the term.
    pub fn encode<W: io::Write>(&self, writer: W) -> EncodeResult {
        codec::Encoder::new(writer).encode(self)
    }

    pub fn as_match<'a, P>(&'a self, pattern: P) -> pattern::Result<P::Output>
    where
        P: pattern::Pattern<'a>,
    {
        pattern.try_match(self)
    }
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Atom(ref x) => x.fmt(f),
            Term::FixInteger(ref x) => x.fmt(f),
            Term::BigInteger(ref x) => x.fmt(f),
            Term::Float(ref x) => x.fmt(f),
            Term::Pid(ref x) => x.fmt(f),
            Term::Port(ref x) => x.fmt(f),
            Term::Reference(ref x) => x.fmt(f),
            Term::ExternalFun(ref x) => x.fmt(f),
            Term::InternalFun(ref x) => x.fmt(f),
            Term::Binary(ref x) => x.fmt(f),
            Term::BitBinary(ref x) => x.fmt(f),
            Term::List(ref x) => x.fmt(f),
            Term::ImproperList(ref x) => x.fmt(f),
            Term::Tuple(ref x) => x.fmt(f),
            Term::Map(ref x) => x.fmt(f),
        }
    }
}
impl From<Atom> for Term {
    fn from(x: Atom) -> Self {
        Term::Atom(x)
    }
}
impl From<FixInteger> for Term {
    fn from(x: FixInteger) -> Self {
        Term::FixInteger(x)
    }
}
impl From<BigInteger> for Term {
    fn from(x: BigInteger) -> Self {
        Term::BigInteger(x)
    }
}
impl From<Float> for Term {
    fn from(x: Float) -> Self {
        Term::Float(x)
    }
}
impl From<Pid> for Term {
    fn from(x: Pid) -> Self {
        Term::Pid(x)
    }
}
impl From<Port> for Term {
    fn from(x: Port) -> Self {
        Term::Port(x)
    }
}
impl From<Reference> for Term {
    fn from(x: Reference) -> Self {
        Term::Reference(x)
    }
}
impl From<ExternalFun> for Term {
    fn from(x: ExternalFun) -> Self {
        Term::ExternalFun(x)
    }
}
impl From<InternalFun> for Term {
    fn from(x: InternalFun) -> Self {
        Term::InternalFun(x)
    }
}
impl From<Binary> for Term {
    fn from(x: Binary) -> Self {
        Term::Binary(x)
    }
}
impl From<BitBinary> for Term {
    fn from(x: BitBinary) -> Self {
        Term::BitBinary(x)
    }
}
impl From<List> for Term {
    fn from(x: List) -> Self {
        Term::List(x)
    }
}
impl From<ImproperList> for Term {
    fn from(x: ImproperList) -> Self {
        Term::ImproperList(x)
    }
}
impl From<Tuple> for Term {
    fn from(x: Tuple) -> Self {
        Term::Tuple(x)
    }
}
impl From<Map> for Term {
    fn from(x: Map) -> Self {
        Term::Map(x)
    }
}

/// Atom.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Atom {
    /// The name of the atom.
    pub name: String,
}
impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "'{}'",
            self.name.replace('\\', "\\\\").replace('\'', "\\'")
        )
    }
}
impl<'a> From<&'a str> for Atom {
    fn from(name: &'a str) -> Self {
        Atom {
            name: name.to_string(),
        }
    }
}
impl From<String> for Atom {
    fn from(name: String) -> Self {
        Atom { name }
    }
}

/// Fixed width integer.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FixInteger {
    /// The value of the integer
    pub value: i32,
}
impl fmt::Display for FixInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl From<u8> for FixInteger {
    fn from(value: u8) -> Self {
        FixInteger {
            value: i32::from(value),
        }
    }
}
impl From<i8> for FixInteger {
    fn from(value: i8) -> Self {
        FixInteger {
            value: i32::from(value),
        }
    }
}
impl From<u16> for FixInteger {
    fn from(value: u16) -> Self {
        FixInteger {
            value: i32::from(value),
        }
    }
}
impl From<i16> for FixInteger {
    fn from(value: i16) -> Self {
        FixInteger {
            value: i32::from(value),
        }
    }
}
impl From<i32> for FixInteger {
    fn from(value: i32) -> Self {
        FixInteger { value }
    }
}

/// Multiple precision integer.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BigInteger {
    /// The value of the integer
    pub value: BigInt,
}
impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl From<i8> for BigInteger {
    fn from(value: i8) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<u8> for BigInteger {
    fn from(value: u8) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<i16> for BigInteger {
    fn from(value: i16) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<u16> for BigInteger {
    fn from(value: u16) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<i32> for BigInteger {
    fn from(value: i32) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<u32> for BigInteger {
    fn from(value: u32) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<i64> for BigInteger {
    fn from(value: i64) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<u64> for BigInteger {
    fn from(value: u64) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<isize> for BigInteger {
    fn from(value: isize) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl From<usize> for BigInteger {
    fn from(value: usize) -> Self {
        BigInteger {
            value: BigInt::from(value),
        }
    }
}
impl<'a> From<&'a FixInteger> for BigInteger {
    fn from(i: &FixInteger) -> Self {
        BigInteger {
            value: BigInt::from(i.value),
        }
    }
}

/// Floating point number
#[derive(Debug, Clone)]
pub struct Float {
    /// The value of the number
    pub value: f64,
}
impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl TryFrom<f32> for Float {
    type Error = DecodeError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Float {
                value: f64::from(value),
            })
        } else {
            Err(DecodeError::NonFiniteFloat)
        }
    }
}
impl TryFrom<f64> for Float {
    type Error = DecodeError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Float { value })
        } else {
            Err(DecodeError::NonFiniteFloat)
        }
    }
}
impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        ordered_float::OrderedFloat(self.value) == ordered_float::OrderedFloat(other.value)
    }
}
impl Eq for Float {}
impl std::hash::Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ordered_float::OrderedFloat(self.value).hash(state);
    }
}

/// Process Identifier.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Pid {
    pub node: Atom,
    pub id: u32,
    pub serial: u32,
    pub creation: u32,
}
impl Pid {
    pub fn new<T>(node: T, id: u32, serial: u32, creation: u32) -> Self
    where
        Atom: From<T>,
    {
        Pid {
            node: Atom::from(node),
            id,
            serial,
            creation,
        }
    }
}
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}.{}.{}>", self.node, self.id, self.serial)
    }
}
// TODO: delete
impl<'a> From<(&'a str, u32, u32)> for Pid {
    fn from((node, id, serial): (&'a str, u32, u32)) -> Self {
        Pid {
            node: Atom::from(node),
            id,
            serial,
            creation: 0,
        }
    }
}

/// Port.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Port {
    pub node: Atom,
    pub id: u32,
    pub creation: u32,
}
impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#Port<{}.{}>", self.node, self.id)
    }
}
impl<'a> From<(&'a str, u32)> for Port {
    fn from((node, id): (&'a str, u32)) -> Self {
        Port {
            node: Atom::from(node),
            id,
            creation: 0,
        }
    }
}

/// Reference.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Reference {
    pub node: Atom,
    pub id: Vec<u32>,
    pub creation: u32,
}
impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#Ref<{}", self.node)?;
        for n in &self.id {
            write!(f, ".{}", n)?;
        }
        write!(f, ">")
    }
}
impl<'a> From<(&'a str, u32)> for Reference {
    fn from((node, id): (&'a str, u32)) -> Self {
        Reference {
            node: Atom::from(node),
            id: vec![id],
            creation: 0,
        }
    }
}
impl<'a> From<(&'a str, Vec<u32>)> for Reference {
    fn from((node, id): (&'a str, Vec<u32>)) -> Self {
        Reference {
            node: Atom::from(node),
            id,
            creation: 0,
        }
    }
}

/// External Function.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExternalFun {
    pub module: Atom,
    pub function: Atom,
    pub arity: u8,
}
impl fmt::Display for ExternalFun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fun {}:{}/{}", self.module, self.function, self.arity)
    }
}
impl<'a, 'b> From<(&'a str, &'b str, u8)> for ExternalFun {
    fn from((module, function, arity): (&'a str, &'b str, u8)) -> Self {
        ExternalFun {
            module: Atom::from(module),
            function: Atom::from(function),
            arity,
        }
    }
}

/// Internal Function.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum InternalFun {
    /// Old representation.
    Old {
        module: Atom,
        pid: Pid,
        free_vars: Vec<Term>,
        index: i32,
        uniq: i32,
    },
    /// New representation.
    New {
        module: Atom,
        arity: u8,
        pid: Pid,
        free_vars: Vec<Term>,
        index: u32,
        uniq: [u8; 16],
        old_index: i32,
        old_uniq: i32,
    },
}
impl fmt::Display for InternalFun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InternalFun::Old {
                ref module,
                index,
                uniq,
                ..
            } => write!(f, "#Fun<{}.{}.{}>", module, index, uniq),
            InternalFun::New {
                ref module,
                index,
                uniq,
                ..
            } => {
                use num::bigint::Sign;
                let uniq = BigInt::from_bytes_be(Sign::Plus, &uniq);
                write!(f, "#Fun<{}.{}.{}>", module, index, uniq)
            }
        }
    }
}

/// Binary.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Binary {
    pub bytes: Vec<u8>,
}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<<")?;
        for (i, b) in self.bytes.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", b)?;
        }
        write!(f, ">>")?;
        Ok(())
    }
}
impl<'a> From<&'a [u8]> for Binary {
    fn from(bytes: &'a [u8]) -> Self {
        Binary {
            bytes: Vec::from(bytes),
        }
    }
}
impl From<Vec<u8>> for Binary {
    fn from(bytes: Vec<u8>) -> Self {
        Binary { bytes }
    }
}

/// Bit string.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BitBinary {
    pub bytes: Vec<u8>,
    pub tail_bits_size: u8,
}
impl fmt::Display for BitBinary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<<")?;
        for (i, b) in self.bytes.iter().enumerate() {
            if i == self.bytes.len() - 1 && self.tail_bits_size == 0 {
                break;
            }
            if i != 0 {
                write!(f, ",")?;
            }
            if i == self.bytes.len() - 1 && self.tail_bits_size < 8 {
                write!(f, "{}:{}", b, self.tail_bits_size)?;
            } else {
                write!(f, "{}", b)?;
            }
        }
        write!(f, ">>")?;
        Ok(())
    }
}
impl From<Binary> for BitBinary {
    fn from(binary: Binary) -> Self {
        BitBinary {
            bytes: binary.bytes,
            tail_bits_size: 8,
        }
    }
}
impl From<(Vec<u8>, u8)> for BitBinary {
    fn from((bytes, tail_bits_size): (Vec<u8>, u8)) -> Self {
        BitBinary {
            bytes,
            tail_bits_size,
        }
    }
}

/// List.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct List {
    pub elements: Vec<Term>,
}
impl List {
    /// Returns a nil value (i.e., an empty list).
    pub fn nil() -> Self {
        List {
            elements: Vec::new(),
        }
    }

    /// Returns `true` if it is nil value, otherwise `false`.
    pub fn is_nil(&self) -> bool {
        self.elements.is_empty()
    }
}
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", x)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
impl From<Vec<Term>> for List {
    fn from(elements: Vec<Term>) -> Self {
        List { elements }
    }
}

/// Improper list.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ImproperList {
    pub elements: Vec<Term>,
    pub last: Box<Term>,
}
impl fmt::Display for ImproperList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", x)?;
        }
        write!(f, "|{}", self.last)?;
        write!(f, "]")?;
        Ok(())
    }
}
impl From<(Vec<Term>, Term)> for ImproperList {
    fn from((elements, last): (Vec<Term>, Term)) -> Self {
        ImproperList {
            elements,
            last: Box::new(last),
        }
    }
}

/// Tuple.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tuple {
    pub elements: Vec<Term>,
}
impl Tuple {
    pub fn nil() -> Self {
        Tuple {
            elements: Vec::new(),
        }
    }
}
impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", x)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
impl From<Vec<Term>> for Tuple {
    fn from(elements: Vec<Term>) -> Self {
        Tuple { elements }
    }
}

/// Map.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Map {
    pub entries: Vec<(Term, Term)>,
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{{")?;
        for (i, &(ref k, ref v)) in self.entries.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}=>{}", k, v)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
impl From<Vec<(Term, Term)>> for Map {
    fn from(entries: Vec<(Term, Term)>) -> Self {
        Map { entries }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::any;
    use crate::pattern::U8;

    #[test]
    fn it_works() {
        let t = Term::from(Atom::from("hoge"));
        t.as_match("hoge").unwrap();

        let t = Term::from(Tuple::from(vec![
            Term::from(Atom::from("foo")),
            Term::from(Atom::from("bar")),
        ]));
        let (_, v) = t.as_match(("foo", any::<Atom>())).unwrap();
        assert_eq!("bar", v.name);

        let t = Term::from(Tuple::from(vec![
            Term::from(Atom::from("foo")),
            Term::from(Atom::from("bar")),
            Term::from(Tuple::from(vec![Term::from(Atom::from("bar"))])),
        ]));
        assert!(t.as_match(("foo", "bar", "baz")).is_err());

        let t = Term::from(FixInteger::from(8));
        t.as_match(U8).unwrap();
    }
}
