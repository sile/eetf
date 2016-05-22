extern crate num;
extern crate byteorder;

use std::fmt;
use std::io;
use std::convert;
use num::bigint::BigInt;

mod codec;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Atom(Atom),
    FixInteger(FixInteger),
    BigInteger(BigInteger),
    Float(Float),
    Pid(Pid),
    Port(Port),
}
//     Reference(Reference),
//     ExternalFun(ExternalFun),
//     InternalFun(InternalFun),
//     Binary(Binary),
//     BitStr(BitStr),
//     List(List),
//     ImproperList(ImproperList),
//     Tuple(Tuple),
//     Map(Map),
// }
impl Term {
    pub fn decode<R: io::Read>(reader: R) -> io::Result<Self> {
        codec::Decoder::new(reader).decode()
    }
    pub fn encode<W: io::Write>(&self, writer: W) -> io::Result<()> {
        codec::Encoder::new(writer).encode(self)
    }
    pub fn as_atom(&self) -> Option<&Atom> {
        if let Term::Atom(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_fix_integer(&self) -> Option<&FixInteger> {
        if let Term::FixInteger(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_big_integer(&self) -> Option<&BigInteger> {
        if let Term::BigInteger(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<&Float> {
        if let Term::Float(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_pid(&self) -> Option<&Pid> {
        if let Term::Pid(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn as_port(&self) -> Option<&Port> {
        if let Term::Port(ref x) = *self {
            Some(x)
        } else {
            None
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

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
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

#[derive(Debug, PartialEq, Clone)]
pub struct FixInteger {
    pub value: i64,
}
impl fmt::Display for FixInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}
impl convert::From<i64> for FixInteger {
    fn from(value: i64) -> Self {
        FixInteger { value: value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BigInteger {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
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
