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
extern crate num;
extern crate byteorder;
extern crate flate2;

use std::fmt;
use std::io;
use std::convert;
use num::bigint::BigInt;

mod codec;

/// Term.
#[derive(Debug, PartialEq, Clone)]
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
    pub fn decode<R: io::Read>(reader: R) -> io::Result<Self> {
        codec::Decoder::new(reader).decode()
    }

    /// Encodes the term.
    pub fn encode<W: io::Write>(&self, writer: W) -> io::Result<()> {
        codec::Encoder::new(writer).encode(self)
    }

    /// Extracts the atom value if it is an atom term.
    pub fn as_atom(&self) -> Option<&Atom> {
        if let Term::Atom(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the atom value if it is an atom term.
    pub fn into_atom(self) -> Result<Atom, Term> {
        if let Term::Atom(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the fix integer value if it is a fix integer term.
    pub fn as_fix_integer(&self) -> Option<&FixInteger> {
        if let Term::FixInteger(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the fix integer value if it is a fix integer term.
    pub fn into_fix_integer(self) -> Result<FixInteger, Term> {
        if let Term::FixInteger(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the big integer value if it is a big integer term.
    pub fn as_big_integer(&self) -> Option<&BigInteger> {
        if let Term::BigInteger(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the big integer value if it is a big integer term.
    pub fn into_big_integer(self) -> Result<BigInteger, Term> {
        if let Term::BigInteger(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the float value if it is a float term.
    pub fn as_float(&self) -> Option<&Float> {
        if let Term::Float(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the float value if it is a float term.
    pub fn into_float(self) -> Result<Float, Term> {
        if let Term::Float(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the pid value if it is a pid term.
    pub fn as_pid(&self) -> Option<&Pid> {
        if let Term::Pid(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the pid value if it is a pid term.
    pub fn into_pid(self) -> Result<Pid, Term> {
        if let Term::Pid(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the port value if it is a port term.
    pub fn as_port(&self) -> Option<&Port> {
        if let Term::Port(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the port value if it is a port term.
    pub fn into_port(self) -> Result<Port, Term> {
        if let Term::Port(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the reference value if it is a reference term.
    pub fn as_reference(&self) -> Option<&Reference> {
        if let Term::Reference(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the reference value if it is a reference term.
    pub fn into_reference(self) -> Result<Reference, Term> {
        if let Term::Reference(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the external functionif it is an external function term.
    pub fn as_external_fun(&self) -> Option<&ExternalFun> {
        if let Term::ExternalFun(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the external function if it is an external function term.
    pub fn into_external_fun(self) -> Result<ExternalFun, Term> {
        if let Term::ExternalFun(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the internal function if it is an internal function term.
    pub fn as_internal_fun(&self) -> Option<&InternalFun> {
        if let Term::InternalFun(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the internal function if it is an internal function term.
    pub fn into_internal_fun(self) -> Result<InternalFun, Term> {
        if let Term::InternalFun(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the binary value if it is a binary term.
    pub fn as_binary(&self) -> Option<&Binary> {
        if let Term::Binary(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the binary value if it is a binary term.
    pub fn into_binary(self) -> Result<Binary, Term> {
        if let Term::Binary(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the bitstring value if it is a bitstring term.
    pub fn as_bit_binary(&self) -> Option<&BitBinary> {
        if let Term::BitBinary(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the bitstring value if it is a bitstring term.
    pub fn into_bit_binary(self) -> Result<BitBinary, Term> {
        if let Term::BitBinary(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the list value if it is a list term.
    pub fn as_list(&self) -> Option<&List> {
        if let Term::List(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the list value if it is a list term.
    pub fn into_list(self) -> Result<List, Term> {
        if let Term::List(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the improper list value if it is an improper list term.
    pub fn as_improper_list(&self) -> Option<&ImproperList> {
        if let Term::ImproperList(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the improper list value if it is an improper list term.
    pub fn into_improper_list(self) -> Result<ImproperList, Term> {
        if let Term::ImproperList(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the tuple value if it is a tuple term.
    pub fn as_tuple(&self) -> Option<&Tuple> {
        if let Term::Tuple(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the tuple value if it is a tuple term.
    pub fn into_tuple(self) -> Result<Tuple, Term> {
        if let Term::Tuple(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }

    /// Extracts the map value if it is a map term.
    pub fn as_map(&self) -> Option<&Map> {
        if let Term::Map(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }

    /// Converts the term to the map value if it is a map term.
    pub fn into_map(self) -> Result<Map, Term> {
        if let Term::Map(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
impl convert::From<Atom> for Term {
    fn from(x: Atom) -> Self {
        Term::Atom(x)
    }
}
impl convert::From<FixInteger> for Term {
    fn from(x: FixInteger) -> Self {
        Term::FixInteger(x)
    }
}
impl convert::From<BigInteger> for Term {
    fn from(x: BigInteger) -> Self {
        Term::BigInteger(x)
    }
}
impl convert::From<Float> for Term {
    fn from(x: Float) -> Self {
        Term::Float(x)
    }
}
impl convert::From<Pid> for Term {
    fn from(x: Pid) -> Self {
        Term::Pid(x)
    }
}
impl convert::From<Port> for Term {
    fn from(x: Port) -> Self {
        Term::Port(x)
    }
}
impl convert::From<Reference> for Term {
    fn from(x: Reference) -> Self {
        Term::Reference(x)
    }
}
impl convert::From<ExternalFun> for Term {
    fn from(x: ExternalFun) -> Self {
        Term::ExternalFun(x)
    }
}
impl convert::From<InternalFun> for Term {
    fn from(x: InternalFun) -> Self {
        Term::InternalFun(x)
    }
}
impl convert::From<Binary> for Term {
    fn from(x: Binary) -> Self {
        Term::Binary(x)
    }
}
impl convert::From<BitBinary> for Term {
    fn from(x: BitBinary) -> Self {
        Term::BitBinary(x)
    }
}
impl convert::From<List> for Term {
    fn from(x: List) -> Self {
        Term::List(x)
    }
}
impl convert::From<ImproperList> for Term {
    fn from(x: ImproperList) -> Self {
        Term::ImproperList(x)
    }
}
impl convert::From<Tuple> for Term {
    fn from(x: Tuple) -> Self {
        Term::Tuple(x)
    }
}
impl convert::From<Map> for Term {
    fn from(x: Map) -> Self {
        Term::Map(x)
    }
}

/// Atom.
#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    /// The name of the atom.
    pub name: String,
}
impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "'{}'",
               self.name.replace("\\", "\\\\").replace("'", "\\'"))
    }
}
impl<'a> convert::From<&'a str> for Atom {
    fn from(name: &'a str) -> Self {
        Atom { name: name.to_string() }
    }
}

/// Fixed width integer.
#[derive(Debug, PartialEq, Clone)]
pub struct FixInteger {
    /// The value of the integer
    pub value: i32,
}
impl fmt::Display for FixInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}
impl convert::From<i32> for FixInteger {
    fn from(value: i32) -> Self {
        FixInteger { value: value }
    }
}

/// Multiple precision integer.
#[derive(Debug, PartialEq, Clone)]
pub struct BigInteger {
    /// The value of the integer
    pub value: BigInt,
}
impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}
impl convert::From<i64> for BigInteger {
    fn from(value: i64) -> Self {
        BigInteger { value: BigInt::from(value) }
    }
}
impl<'a> convert::From<&'a FixInteger> for BigInteger {
    fn from(i: &FixInteger) -> Self {
        BigInteger { value: BigInt::from(i.value) }
    }
}

/// Floating point number
#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    /// The value of the number
    pub value: f64,
}
impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}
impl convert::From<f64> for Float {
    fn from(value: f64) -> Self {
        Float { value: value }
    }
}

/// Process Identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct Pid {
    pub node: Atom,
    pub id: u32,
    pub serial: u32,
    pub creation: u8,
}
impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}.{}.{}>", self.node, self.id, self.serial)
    }
}
impl<'a> convert::From<(&'a str, u32, u32)> for Pid {
    fn from((node, id, serial): (&'a str, u32, u32)) -> Self {
        Pid {
            node: Atom::from(node),
            id: id,
            serial: serial,
            creation: 0,
        }
    }
}

/// Port.
#[derive(Debug, PartialEq, Clone)]
pub struct Port {
    pub node: Atom,
    pub id: u32,
    pub creation: u8,
}
impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "#Port<{}.{}>", self.node, self.id)
    }
}
impl<'a> convert::From<(&'a str, u32)> for Port {
    fn from((node, id): (&'a str, u32)) -> Self {
        Port {
            node: Atom::from(node),
            id: id,
            creation: 0,
        }
    }
}

/// Reference.
#[derive(Debug, PartialEq, Clone)]
pub struct Reference {
    pub node: Atom,
    pub id: Vec<u32>,
    pub creation: u8,
}
impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "#Ref<{}", self.node));
        for n in &self.id {
            try!(write!(f, ".{}", n));
        }
        write!(f, ">")
    }
}
impl<'a> convert::From<(&'a str, u32)> for Reference {
    fn from((node, id): (&'a str, u32)) -> Self {
        Reference {
            node: Atom::from(node),
            id: vec![id],
            creation: 0,
        }
    }
}
impl<'a> convert::From<(&'a str, Vec<u32>)> for Reference {
    fn from((node, id): (&'a str, Vec<u32>)) -> Self {
        Reference {
            node: Atom::from(node),
            id: id,
            creation: 0,
        }
    }
}

/// External Function.
#[derive(Debug, PartialEq, Clone)]
pub struct ExternalFun {
    pub module: Atom,
    pub function: Atom,
    pub arity: u8,
}
impl fmt::Display for ExternalFun {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "fun {}:{}/{}", self.module, self.function, self.arity)
    }
}
impl<'a, 'b> convert::From<(&'a str, &'b str, u8)> for ExternalFun {
    fn from((module, function, arity): (&'a str, &'b str, u8)) -> Self {
        ExternalFun {
            module: Atom::from(module),
            function: Atom::from(function),
            arity: arity,
        }
    }
}

/// Internal Function.
#[derive(Debug, PartialEq, Clone)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            InternalFun::Old { ref module, index, uniq, .. } => {
                write!(f, "#Fun<{}.{}.{}>", module, index, uniq)
            }
            InternalFun::New { ref module, index, uniq, .. } => {
                use num::bigint::Sign;
                let uniq = BigInt::from_bytes_be(Sign::Plus, &uniq);
                write!(f, "#Fun<{}.{}.{}>", module, index, uniq)
            }
        }
    }
}

/// Binary.
#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
    pub bytes: Vec<u8>,
}
impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "<<"));
        for (i, b) in self.bytes.iter().enumerate() {
            if i != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}", b));
        }
        try!(write!(f, ">>"));
        Ok(())
    }
}
impl<'a> convert::From<(&'a [u8])> for Binary {
    fn from(bytes: &'a [u8]) -> Self {
        Binary { bytes: Vec::from(bytes) }
    }
}
impl convert::From<Vec<u8>> for Binary {
    fn from(bytes: Vec<u8>) -> Self {
        Binary { bytes: bytes }
    }
}

/// Bit string.
#[derive(Debug, PartialEq, Clone)]
pub struct BitBinary {
    pub bytes: Vec<u8>,
    pub tail_bits_size: u8,
}
impl fmt::Display for BitBinary {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "<<"));
        for (i, b) in self.bytes.iter().enumerate() {
            if i == self.bytes.len() - 1 && self.tail_bits_size == 0 {
                break;
            }
            if i != 0 {
                try!(write!(f, ","));
            }
            if i == self.bytes.len() - 1 && self.tail_bits_size < 8 {
                try!(write!(f, "{}:{}", b, self.tail_bits_size));
            } else {
                try!(write!(f, "{}", b));
            }
        }
        try!(write!(f, ">>"));
        Ok(())
    }
}
impl convert::From<Binary> for BitBinary {
    fn from(binary: Binary) -> Self {
        BitBinary {
            bytes: binary.bytes,
            tail_bits_size: 8,
        }
    }
}
impl convert::From<(Vec<u8>, u8)> for BitBinary {
    fn from((bytes, tail_bits_size): (Vec<u8>, u8)) -> Self {
        BitBinary {
            bytes: bytes,
            tail_bits_size: tail_bits_size,
        }
    }
}

/// List.
#[derive(Debug, PartialEq, Clone)]
pub struct List {
    pub elements: Vec<Term>,
}
impl List {
    /// Returns a nil value (i.e., an empty list).
    pub fn nil() -> Self {
        List { elements: Vec::new() }
    }

    /// Returns `true` if it is nil value, otherwise `false`.
    pub fn is_nil(&self) -> bool {
        self.elements.is_empty()
    }
}
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "["));
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}", x));
        }
        try!(write!(f, "]"));
        Ok(())
    }
}
impl convert::From<Vec<Term>> for List {
    fn from(elements: Vec<Term>) -> Self {
        List { elements: elements }
    }
}

/// Improper list.
#[derive(Debug, PartialEq, Clone)]
pub struct ImproperList {
    pub elements: Vec<Term>,
    pub last: Box<Term>,
}
impl fmt::Display for ImproperList {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "["));
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}", x));
        }
        try!(write!(f, "|{}", self.last));
        try!(write!(f, "]"));
        Ok(())
    }
}
impl convert::From<(Vec<Term>, Term)> for ImproperList {
    fn from((elements, last): (Vec<Term>, Term)) -> Self {
        ImproperList {
            elements: elements,
            last: Box::new(last),
        }
    }
}

/// Tuple.
#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    pub elements: Vec<Term>,
}
impl Tuple {
    pub fn nil() -> Self {
        Tuple { elements: Vec::new() }
    }
}
impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{{"));
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}", x));
        }
        try!(write!(f, "}}"));
        Ok(())
    }
}
impl convert::From<Vec<Term>> for Tuple {
    fn from(elements: Vec<Term>) -> Self {
        Tuple { elements: elements }
    }
}

/// Map.
#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    pub entries: Vec<(Term, Term)>,
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "#{{"));
        for (i, &(ref k, ref v)) in self.entries.iter().enumerate() {
            if i != 0 {
                try!(write!(f, ","));
            }
            try!(write!(f, "{}=>{}", k, v));
        }
        try!(write!(f, "}}"));
        Ok(())
    }
}
impl convert::From<Vec<(Term, Term)>> for Map {
    fn from(entries: Vec<(Term, Term)>) -> Self {
        Map { entries: entries }
    }
}
