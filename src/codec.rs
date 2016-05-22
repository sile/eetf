use std;
use std::str;
use std::io;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use num::bigint::BigInt;
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
            NEW_FLOAT_EXT => self.decode_new_float_ext(),
            BIT_BINARY_EXT => unimplemented!(),
            COMPRESSED_TERM => unimplemented!(),
            ATOM_CACHE_REF => unimplemented!(),
            SMALL_INTEGER_EXT => self.decode_small_integer_ext(),
            INTEGER_EXT => self.decode_integer_ext(),
            FLOAT_EXT => self.decode_float_ext(),
            ATOM_EXT => self.decode_atom_ext(),
            REFERENCE_EXT => unimplemented!(),
            PORT_EXT => self.decode_port_ext(),
            PID_EXT => self.decode_pid_ext(),
            SMALL_TUPLE_EXT => unimplemented!(),
            LARGE_TUPLE_EXT => unimplemented!(),
            NIL_EXT => unimplemented!(),
            STRING_EXT => unimplemented!(),
            LIST_EXT => unimplemented!(),
            BINARY_EXT => unimplemented!(),
            SMALL_BIG_EXT => self.decode_small_big_ext(),
            LARGE_BIG_EXT => self.decode_large_big_ext(),
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
    fn decode_pid_ext(&mut self) -> DecodeResult {
        let node = try!(self.decode_term().and_then(|t| {
            if let Some(a) = t.as_atom() {
                Ok(a.clone())
            } else {
                aux::invalid_data_error(format!("Node must be an atom: value={}", t))
            }
        }));
        Ok(Term::from(Pid {
            node: node,
            id: try!(self.reader.read_u32::<BigEndian>()),
            serial: try!(self.reader.read_u32::<BigEndian>()),
            creation: try!(self.reader.read_u8()),
        }))
    }
    fn decode_port_ext(&mut self) -> DecodeResult {
        let node = try!(self.decode_term().and_then(|t| {
            if let Some(a) = t.as_atom() {
                Ok(a.clone())
            } else {
                aux::invalid_data_error(format!("Node must be an atom: value={}", t))
            }
        }));
        Ok(Term::from(Port {
            node: node,
            id: try!(self.reader.read_u32::<BigEndian>()),
            creation: try!(self.reader.read_u8()),
        }))
    }
    fn decode_new_float_ext(&mut self) -> DecodeResult {
        let value = try!(self.reader.read_f64::<BigEndian>());
        Ok(Term::from(Float::from(value)))
    }
    fn decode_float_ext(&mut self) -> DecodeResult {
        // FIXME: Implement
        unimplemented!()
    }
    fn decode_small_integer_ext(&mut self) -> DecodeResult {
        let value = try!(self.reader.read_u8());
        Ok(Term::from(FixInteger::from(value as i64)))
    }
    fn decode_integer_ext(&mut self) -> DecodeResult {
        let value = try!(self.reader.read_i32::<BigEndian>());
        Ok(Term::from(FixInteger::from(value as i64)))
    }
    fn decode_small_big_ext(&mut self) -> DecodeResult {
        let count = try!(self.reader.read_u8()) as usize;
        let sign = try!(self.reader.read_u8());
        self.buf.resize(count, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let value = BigInt::from_bytes_le(try!(aux::byte_to_sign(sign)), &self.buf);
        Ok(Term::from(BigInteger { value: value }))
    }
    fn decode_large_big_ext(&mut self) -> DecodeResult {
        let count = try!(self.reader.read_u32::<BigEndian>()) as usize;
        let sign = try!(self.reader.read_u8());
        self.buf.resize(count, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let value = BigInt::from_bytes_le(try!(aux::byte_to_sign(sign)), &self.buf);
        Ok(Term::from(BigInteger { value: value }))
    }
    fn decode_atom_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u16::<BigEndian>());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(aux::latin1_bytes_to_string(&self.buf));
        Ok(Term::from(Atom { name: name }))
    }
    fn decode_small_atom_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u8());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(aux::latin1_bytes_to_string(&self.buf));
        Ok(Term::from(Atom { name: name }))
    }
    fn decode_atom_utf8_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u16::<BigEndian>());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(str::from_utf8(&self.buf)
            .or_else(|e| aux::invalid_data_error(e.to_string())));
        Ok(Term::from(Atom::from(name)))
    }
    fn decode_small_atom_utf8_ext(&mut self) -> DecodeResult {
        let len = try!(self.reader.read_u8());
        self.buf.resize(len as usize, 0);
        try!(self.reader.read_exact(&mut self.buf));
        let name = try!(str::from_utf8(&self.buf)
            .or_else(|e| aux::invalid_data_error(e.to_string())));
        Ok(Term::from(Atom::from(name)))
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
            Term::FixInteger(ref x) => self.encode_fix_integer(x),
            Term::BigInteger(ref x) => self.encode_big_integer(x),
            Term::Float(ref x) => self.encode_float(x),
            Term::Pid(ref x) => self.encode_pid(x),
            Term::Port(ref x) => self.encode_port(x),
        }
    }
    fn encode_float(&mut self, x: &Float) -> EncodeResult {
        try!(self.writer.write_u8(NEW_FLOAT_EXT));
        try!(self.writer.write_f64::<BigEndian>(x.value));
        Ok(())
    }
    fn encode_atom(&mut self, x: &Atom) -> EncodeResult {
        if x.name.len() > 0xFFFF {
            return aux::invalid_data_error(format!("Too long atom name: length={}", x.name.len()));
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
    fn encode_fix_integer(&mut self, x: &FixInteger) -> EncodeResult {
        if 0 <= x.value && x.value <= std::u8::MAX as i64 {
            try!(self.writer.write_u8(SMALL_INTEGER_EXT));
            try!(self.writer.write_u8(x.value as u8));
        } else if std::i32::MIN as i64 <= x.value && x.value <= std::i32::MAX as i64 {
            try!(self.writer.write_u8(INTEGER_EXT));
            try!(self.writer.write_i32::<BigEndian>(x.value as i32));
        } else {
            try!(self.encode_big_integer(&BigInteger::from(x)));
        }
        Ok(())
    }
    fn encode_big_integer(&mut self, x: &BigInteger) -> EncodeResult {
        let (sign, bytes) = x.value.to_bytes_le();
        if bytes.len() <= std::u8::MAX as usize {
            try!(self.writer.write_u8(SMALL_BIG_EXT));
            try!(self.writer.write_u8(bytes.len() as u8));
        } else if bytes.len() <= std::u32::MAX as usize {
            try!(self.writer.write_u8(LARGE_BIG_EXT));
            try!(self.writer.write_u32::<BigEndian>(bytes.len() as u32));
        } else {
            return aux::invalid_data_error(format!("Too large integer: {} bytes", bytes.len()));
        }
        try!(self.writer.write_u8(aux::sign_to_byte(sign)));
        try!(self.writer.write_all(&bytes));
        Ok(())
    }
    fn encode_pid(&mut self, x: &Pid) -> EncodeResult {
        try!(self.writer.write_u8(PID_EXT));
        try!(self.encode_atom(&x.node));
        try!(self.writer.write_u32::<BigEndian>(x.id));
        try!(self.writer.write_u32::<BigEndian>(x.serial));
        try!(self.writer.write_u8(x.creation));
        Ok(())
    }
    fn encode_port(&mut self, x: &Port) -> EncodeResult {
        try!(self.writer.write_u8(PORT_EXT));
        try!(self.encode_atom(&x.node));
        try!(self.writer.write_u32::<BigEndian>(x.id));
        try!(self.writer.write_u8(x.creation));
        Ok(())
    }
}

mod aux {
    use std::str;
    use std::io;
    use num::bigint::Sign;

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
    pub fn byte_to_sign(b: u8) -> io::Result<Sign> {
        match b {
            0 => Ok(Sign::Plus),
            1 => Ok(Sign::Minus),
            _ => invalid_data_error(format!("A sign value must be 0 or 1: value={}", b)),
        }
    }
    pub fn sign_to_byte(sign: Sign) -> u8 {
        if sign == Sign::Minus {
            1
        } else {
            0
        }
    }
}
