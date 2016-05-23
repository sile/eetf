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
    Reference(Reference),
    ExternalFun(ExternalFun),
    InternalFun(InternalFun),
}
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
    pub fn into_atom(self) -> Result<Atom, Term> {
        if let Term::Atom(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_fix_integer(&self) -> Option<&FixInteger> {
        if let Term::FixInteger(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_fix_integer(self) -> Result<FixInteger, Term> {
        if let Term::FixInteger(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_big_integer(&self) -> Option<&BigInteger> {
        if let Term::BigInteger(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_big_integer(self) -> Result<BigInteger, Term> {
        if let Term::BigInteger(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_float(&self) -> Option<&Float> {
        if let Term::Float(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_float(self) -> Result<Float, Term> {
        if let Term::Float(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_pid(&self) -> Option<&Pid> {
        if let Term::Pid(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_pid(self) -> Result<Pid, Term> {
        if let Term::Pid(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_port(&self) -> Option<&Port> {
        if let Term::Port(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_port(self) -> Result<Port, Term> {
        if let Term::Port(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_reference(&self) -> Option<&Reference> {
        if let Term::Reference(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_reference(self) -> Result<Reference, Term> {
        if let Term::Reference(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_external_fun(&self) -> Option<&ExternalFun> {
        if let Term::ExternalFun(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_external_fun(self) -> Result<ExternalFun, Term> {
        if let Term::ExternalFun(x) = self {
            Ok(x)
        } else {
            Err(self)
        }
    }
    pub fn as_internal_fun(&self) -> Option<&InternalFun> {
        if let Term::InternalFun(ref x) = *self {
            Some(x)
        } else {
            None
        }
    }
    pub fn into_internal_fun(self) -> Result<InternalFun, Term> {
        if let Term::InternalFun(x) = self {
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

#[derive(Debug, PartialEq, Clone)]
pub enum InternalFun {
    Old {
        module: Atom,
        pid: Pid,
        free_vars: Vec<Term>,
        index: i32,
        uniq: i32,
    },
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
