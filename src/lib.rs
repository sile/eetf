extern crate num;
extern crate byteorder;

use std::fmt;
use std::io;

mod codec;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Atom(Atom),
    Nil,
}
//     Nil,
//     List(List),
//     ImproperList(ImproperList),
//     Tuple(Tuple),
//     Map(Map),
//     Binary(Binary),
//     BitStr(BitStr),
//     Float(Float),
//     FixInt(FixInt),
//     BigInt(BigInt),
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
        unimplemented!()
    }
    pub fn as_atom(&self) -> Option<&Atom> {
        if let &Term::Atom(ref x) = self {
            Some(x)
        } else {
            None
        }
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
