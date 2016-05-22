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
}
//     List(List),
//     ImproperList(ImproperList),
//     Tuple(Tuple),
//     Map(Map),
//     Binary(Binary),
//     BitStr(BitStr),

//     Float(Float),
//     Pid(Pid),
//     Port(Port),
//     Reference(Reference),
//     ExternalFun(ExternalFun),
//     InternalFun(InternalFun),
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
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Term::Atom(ref x) => x.fmt(f),
            Term::FixInteger(ref x) => x.fmt(f),
            Term::BigInteger(ref x) => x.fmt(f),
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
