use std::str;
use std::io;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use super::*;

const VERSION: u8 = 131;

const DISTRIBUTION_HEADER: u8 = 68;
const NEW_FLOAT_EXT: u8 = 70;
const BIT_BINARY_EXT: u8 = 77;
const COMPRESSED_TERM: u8 = 80;
const ATOM_CACHE_REF: u8 = 82;
const SMALL_INTEGER_EXT: u8 = 97;
const INTEGER_EXT: u8 = 98;
const FLOAT_EXT: u8 = 99;
const ATOM_EXT: u8 = 100;
const REFERENCE_EXT: u8 = 101;
const PORT_EXT: u8 = 102;
const PID_EXT: u8 = 103;
const SMALL_TUPLE_EXT: u8 = 104;
const LARGE_TUPLE_EXT: u8 = 105;
const NIL_EXT: u8 = 106;
const STRING_EXT: u8 = 107;
const LIST_EXT: u8 = 108;
const BINARY_EXT: u8 = 109;
const SMALL_BIG_EXT: u8 = 110;
const LARGE_BIG_EXT: u8 = 111;
const NEW_FUN_EXT: u8 = 112;
const EXPORT_EXT: u8 = 113;
const NEW_REFERENCE_EXT: u8 = 114;
const SMALL_ATOM_EXT: u8 = 115;
const MAP_EXT: u8 = 116;
const FUN_EXT: u8 = 117;
const ATOM_UTF8_EXT: u8 = 118;
const SMALL_ATOM_UTF8_EXT: u8 = 119;

pub type DecodeResult = io::Result<Term>;
pub struct Decoder<R> {
    reader: R,
    buf: Vec<u8>,
}
impl<R: io::Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Decoder {
            reader: reader,
            buf: Vec::new(),
        }
    }
    pub fn decode(mut self) -> DecodeResult {
        let version = try!(self.reader.read_u8());
        if version != VERSION {
            return aux::invalid_data_error(format!("Unsupported version: {} ", version));
        }
        self.decode_term()
    }
    fn decode_term(&mut self) -> DecodeResult {
        let tag = try!(self.reader.read_u8());
        match tag {
            DISTRIBUTION_HEADER => unimplemented!(),
            NEW_FLOAT_EXT => unimplemented!(),
            BIT_BINARY_EXT => unimplemented!(),
            COMPRESSED_TERM => unimplemented!(),
            ATOM_CACHE_REF => unimplemented!(),
            SMALL_INTEGER_EXT => unimplemented!(),
            INTEGER_EXT => unimplemented!(),
            FLOAT_EXT => unimplemented!(),
            ATOM_EXT => self.decode_atom_ext(),
            REFERENCE_EXT => unimplemented!(),
            PORT_EXT => unimplemented!(),
            PID_EXT => unimplemented!(),
            SMALL_TUPLE_EXT => unimplemented!(),
            LARGE_TUPLE_EXT => unimplemented!(),
            NIL_EXT => unimplemented!(),
            STRING_EXT => unimplemented!(),
            LIST_EXT => unimplemented!(),
            BINARY_EXT => unimplemented!(),
            SMALL_BIG_EXT => unimplemented!(),
            LARGE_BIG_EXT => unimplemented!(),
            NEW_FUN_EXT => unimplemented!(),
            EXPORT_EXT => unimplemented!(),
            NEW_REFERENCE_EXT => unimplemented!(),
            SMALL_ATOM_EXT => self.decode_small_atom_ext(),
            MAP_EXT => unimplemented!(),
            FUN_EXT => unimplemented!(),
            ATOM_UTF8_EXT => self.decode_atom_utf8_ext(),
            SMALL_ATOM_UTF8_EXT => self.decode_small_atom_utf8_ext(),
            _ => aux::invalid_data_error(format!("Unknown tag: {}", tag)),
        }
    }
    fn decode_atom_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u16::<BigEndian>());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(aux::latin1_bytes_to_string(&self.buf));
        Ok(Term::Atom(Atom { name: name }))
    }
    fn decode_small_atom_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u8());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(aux::latin1_bytes_to_string(&self.buf));
        Ok(Term::Atom(Atom { name: name }))
    }
    fn decode_atom_utf8_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u16::<BigEndian>());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(str::from_utf8(&self.buf)
            .or_else(|e| aux::invalid_data_error(e.to_string())));
        Ok(Term::Atom(Atom { name: name.to_string() }))
    }
    fn decode_small_atom_utf8_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u8());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(str::from_utf8(&self.buf)
            .or_else(|e| aux::invalid_data_error(e.to_string())));
        Ok(Term::Atom(Atom { name: name.to_string() }))
    }
}

pub type EncodeResult = io::Result<()>;
pub struct Encoder<W> {
    writer: W,
}
impl<W: io::Write> Encoder<W> {
    pub fn new(writer: W) -> Self {
        Encoder { writer: writer }
    }
    pub fn encode(mut self, term: &Term) -> EncodeResult {
        try!(self.writer.write_u8(VERSION));
        self.encode_term(term)
    }
    fn encode_term(&mut self, term: &Term) -> EncodeResult {
        match *term {
            Term::Atom(ref x) => self.encode_atom(x),
            _ => unimplemented!(),
        }
    }
    fn encode_atom(&mut self, x: &Atom) -> EncodeResult {
        if x.name.len() > 0xFFFF {
            return aux::invalid_input_error(format!("Too long atom name: length={}", x.name.len()));
        }

        let is_ascii = x.name.as_bytes().iter().all(|&c| c < 0x80);
        if is_ascii {
            try!(self.writer.write_u8(ATOM_EXT));
        } else {
            try!(self.writer.write_u8(ATOM_UTF8_EXT));
        }
        try!(self.writer.write_u16::<BigEndian>(x.name.len() as u16));
        try!(self.writer.write_all(x.name.as_bytes()));
        Ok(())
    }
}

mod aux {
    use std::str;
    use std::io;

    pub fn invalid_input_error<T>(message: String) -> io::Result<T> {
        Err(io::Error::new(io::ErrorKind::InvalidInput, message))
    }
    pub fn invalid_data_error<T>(message: String) -> io::Result<T> {
        Err(io::Error::new(io::ErrorKind::InvalidData, message))
    }
    pub fn other_error<T>(message: String) -> io::Result<T> {
        Err(io::Error::new(io::ErrorKind::Other, message))
    }
    pub fn latin1_bytes_to_string(buf: &[u8]) -> io::Result<String> {
        // FIXME: Supports Latin1 characters
        str::from_utf8(buf).or_else(|e| other_error(e.to_string())).map(|s| s.to_string())
    }
}
