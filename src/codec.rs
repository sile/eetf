use std;
use std::str;
use std::io;
use std::io::Write;
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
            BIT_BINARY_EXT => self.decode_bit_binary_ext(),
            COMPRESSED_TERM => unimplemented!(),
            ATOM_CACHE_REF => unimplemented!(),
            SMALL_INTEGER_EXT => self.decode_small_integer_ext(),
            INTEGER_EXT => self.decode_integer_ext(),
            FLOAT_EXT => self.decode_float_ext(),
            ATOM_EXT => self.decode_atom_ext(),
            REFERENCE_EXT => self.decode_reference_ext(),
            PORT_EXT => self.decode_port_ext(),
            PID_EXT => self.decode_pid_ext(),
            SMALL_TUPLE_EXT => self.decode_small_tuple_ext(),
            LARGE_TUPLE_EXT => self.decode_large_tuple_ext(),
            NIL_EXT => self.decode_nil_ext(),
            STRING_EXT => self.decode_string_ext(),
            LIST_EXT => self.decode_list_ext(),
            BINARY_EXT => self.decode_binary_ext(),
            SMALL_BIG_EXT => self.decode_small_big_ext(),
            LARGE_BIG_EXT => self.decode_large_big_ext(),
            NEW_FUN_EXT => self.decode_new_fun_ext(),
            EXPORT_EXT => self.decode_export_ext(),
            NEW_REFERENCE_EXT => self.decode_new_reference_ext(),
            SMALL_ATOM_EXT => self.decode_small_atom_ext(),
            MAP_EXT => self.decode_map_ext(),
            FUN_EXT => self.decode_fun_ext(),
            ATOM_UTF8_EXT => self.decode_atom_utf8_ext(),
            SMALL_ATOM_UTF8_EXT => self.decode_small_atom_utf8_ext(),
            _ => aux::invalid_data_error(format!("Unknown tag: {}", tag)),
        }
    }
    fn decode_nil_ext(&mut self) -> DecodeResult {
        Ok(Term::from(List::nil()))
    }
    fn decode_string_ext(&mut self) -> DecodeResult {
        let size = try!(self.reader.read_u16::<BigEndian>()) as usize;
        let mut elements = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(Term::from(FixInteger::from(try!(self.reader.read_u8()) as i32)));
        }
        Ok(Term::from(List::from(elements)))
    }
    fn decode_list_ext(&mut self) -> DecodeResult {
        let count = try!(self.reader.read_u32::<BigEndian>()) as usize;
        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            elements.push(try!(self.decode_term()));
        }
        let last = try!(self.decode_term());
        if last.as_list().map(|l| l.is_nil()).unwrap_or(false) {
            Ok(Term::from(List::from(elements)))
        } else {
            Ok(Term::from(ImproperList::from((elements, last))))
        }
    }
    fn decode_small_tuple_ext(&mut self) -> DecodeResult {
        let count = try!(self.reader.read_u8()) as usize;
        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            elements.push(try!(self.decode_term()));
        }
        Ok(Term::from(Tuple::from(elements)))
    }
    fn decode_large_tuple_ext(&mut self) -> DecodeResult {
        let count = try!(self.reader.read_u32::<BigEndian>()) as usize;
        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            elements.push(try!(self.decode_term()));
        }
        Ok(Term::from(Tuple::from(elements)))
    }
    fn decode_map_ext(&mut self) -> DecodeResult {
        let count = try!(self.reader.read_u32::<BigEndian>()) as usize;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            let k = try!(self.decode_term());
            let v = try!(self.decode_term());
            entries.push((k, v));
        }
        Ok(Term::from(Map::from(entries)))
    }
    fn decode_binary_ext(&mut self) -> DecodeResult {
        let size = try!(self.reader.read_u32::<BigEndian>()) as usize;
        let mut buf = vec![0; size];
        try!(self.reader.read_exact(&mut buf));
        Ok(Term::from(Binary::from(buf)))
    }
    fn decode_bit_binary_ext(&mut self) -> DecodeResult {
        let size = try!(self.reader.read_u32::<BigEndian>()) as usize;
        let tail_bits_size = try!(self.reader.read_u8());
        let mut buf = vec![0; size];
        try!(self.reader.read_exact(&mut buf));
        if !buf.is_empty() {
            let last = buf[size - 1] >> (8 - tail_bits_size);
            buf[size - 1] = last;
        }
        Ok(Term::from(BitBinary::from((buf, tail_bits_size))))
    }
    fn decode_pid_ext(&mut self) -> DecodeResult {
        let node = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        Ok(Term::from(Pid {
            node: node,
            id: try!(self.reader.read_u32::<BigEndian>()),
            serial: try!(self.reader.read_u32::<BigEndian>()),
            creation: try!(self.reader.read_u8()),
        }))
    }
    fn decode_port_ext(&mut self) -> DecodeResult {
        let node = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        Ok(Term::from(Port {
            node: node,
            id: try!(self.reader.read_u32::<BigEndian>()),
            creation: try!(self.reader.read_u8()),
        }))
    }
    fn decode_reference_ext(&mut self) -> DecodeResult {
        let node = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        Ok(Term::from(Reference {
            node: node,
            id: vec![try!(self.reader.read_u32::<BigEndian>())],
            creation: try!(self.reader.read_u8()),
        }))
    }
    fn decode_new_reference_ext(&mut self) -> DecodeResult {
        let id_count = try!(self.reader.read_u16::<BigEndian>()) as usize;
        let node = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        let creation = try!(self.reader.read_u8());
        let mut id = Vec::with_capacity(id_count);
        for _ in 0..id_count {
            id.push(try!(self.reader.read_u32::<BigEndian>()));
        }
        Ok(Term::from(Reference {
            node: node,
            id: id,
            creation: creation,
        }))
    }
    fn decode_export_ext(&mut self) -> DecodeResult {
        let module = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        let function = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        let arity = try!(self.decode_term().and_then(|t| {
            match t.as_fix_integer() {
                Some(&FixInteger { value }) if 0 <= value && value <= std::u8::MAX as i32 => {
                    Ok(value as u8)
                }
                _ => aux::invalid_data_error(format!("Arity must be an u8: value={}", t)),
            }
        }));
        Ok(Term::from(ExternalFun {
            module: module,
            function: function,
            arity: arity,
        }))
    }
    fn decode_fun_ext(&mut self) -> DecodeResult {
        let num_free = try!(self.reader.read_u32::<BigEndian>());
        let pid = try!(self.decode_term()
            .and_then(|t| {
                t.into_pid().or_else(|t| aux::invalid_data_error(format!("Not a pid: {}", t)))
            }));
        let module = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        let index = try!(self.decode_term().and_then(|t| {
            t.into_fix_integer()
                .or_else(|t| aux::invalid_data_error(format!("Not an integer: {}", t)))
        }));
        let uniq = try!(self.decode_term().and_then(|t| {
            t.into_fix_integer()
                .or_else(|t| aux::invalid_data_error(format!("Not an integer: {}", t)))
        }));
        let mut vars = Vec::with_capacity(num_free as usize);
        for _ in 0..num_free {
            vars.push(try!(self.decode_term()));
        }
        Ok(Term::from(InternalFun::Old {
            module: module,
            pid: pid,
            free_vars: vars,
            index: index.value,
            uniq: uniq.value,
        }))
    }
    fn decode_new_fun_ext(&mut self) -> DecodeResult {
        let _size = try!(self.reader.read_u32::<BigEndian>());
        let arity = try!(self.reader.read_u8());
        let mut uniq = [0; 16];
        try!(self.reader.read_exact(&mut uniq));
        let index = try!(self.reader.read_u32::<BigEndian>());
        let num_free = try!(self.reader.read_u32::<BigEndian>());
        let module = try!(self.decode_term()
            .and_then(|t| {
                t.into_atom().or_else(|t| aux::invalid_data_error(format!("Not an atom: {}", t)))
            }));
        let old_index = try!(self.decode_term().and_then(|t| {
            t.into_fix_integer()
                .or_else(|t| aux::invalid_data_error(format!("Not an integer: {}", t)))
        }));
        let old_uniq = try!(self.decode_term().and_then(|t| {
            t.into_fix_integer()
                .or_else(|t| aux::invalid_data_error(format!("Not an integer: {}", t)))
        }));
        let pid = try!(self.decode_term()
            .and_then(|t| {
                t.into_pid().or_else(|t| aux::invalid_data_error(format!("Not a pid: {}", t)))
            }));
        let mut vars = Vec::with_capacity(num_free as usize);
        for _ in 0..num_free {
            vars.push(try!(self.decode_term()));
        }
        Ok(Term::from(InternalFun::New {
            module: module,
            arity: arity,
            pid: pid,
            free_vars: vars,
            index: index,
            uniq: uniq,
            old_index: old_index.value,
            old_uniq: old_uniq.value,
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
        Ok(Term::from(FixInteger::from(value as i32)))
    }
    fn decode_integer_ext(&mut self) -> DecodeResult {
        let value = try!(self.reader.read_i32::<BigEndian>());
        Ok(Term::from(FixInteger::from(value)))
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
            Term::Reference(ref x) => self.encode_reference(x),
            Term::ExternalFun(ref x) => self.encode_external_fun(x),
            Term::InternalFun(ref x) => self.encode_internal_fun(x),
            Term::Binary(ref x) => self.encode_binary(x),
            Term::BitBinary(ref x) => self.encode_bit_binary(x),
            Term::List(ref x) => self.encode_list(x),
            Term::ImproperList(ref x) => self.encode_improper_list(x),
            Term::Tuple(ref x) => self.encode_tuple(x),
            Term::Map(ref x) => self.encode_map(x),
        }
    }
    fn encode_nil(&mut self) -> EncodeResult {
        try!(self.writer.write_u8(NIL_EXT));
        Ok(())
    }
    fn encode_list(&mut self, x: &List) -> EncodeResult {
        let to_byte = |e: &Term| {
            e.as_fix_integer().and_then(|&FixInteger { value: i }| if i < 0x100 {
                Some(i as u8)
            } else {
                None
            })
        };
        if !x.elements.is_empty() && x.elements.len() <= std::u16::MAX as usize &&
           x.elements.iter().all(|e| to_byte(e).is_some()) {
            try!(self.writer.write_u8(STRING_EXT));
            try!(self.writer.write_u16::<BigEndian>(x.elements.len() as u16));
            for b in x.elements.iter().map(|e| to_byte(e).unwrap()) {
                try!(self.writer.write_u8(b));
            }
        } else {
            if !x.is_nil() {
                try!(self.writer.write_u8(LIST_EXT));
                try!(self.writer.write_u32::<BigEndian>(x.elements.len() as u32));
                for e in &x.elements {
                    try!(self.encode_term(e));
                }
            }
            try!(self.encode_nil());
        }
        Ok(())
    }
    fn encode_improper_list(&mut self, x: &ImproperList) -> EncodeResult {
        try!(self.writer.write_u8(LIST_EXT));
        try!(self.writer.write_u32::<BigEndian>(x.elements.len() as u32));
        for e in &x.elements {
            try!(self.encode_term(e));
        }
        try!(self.encode_term(&x.last));
        Ok(())
    }
    fn encode_tuple(&mut self, x: &Tuple) -> EncodeResult {
        if x.elements.len() < 0x100 {
            try!(self.writer.write_u8(SMALL_TUPLE_EXT));
            try!(self.writer.write_u8(x.elements.len() as u8));
        } else {
            try!(self.writer.write_u8(LARGE_TUPLE_EXT));
            try!(self.writer.write_u32::<BigEndian>(x.elements.len() as u32));
        }
        for e in &x.elements {
            try!(self.encode_term(e));
        }
        Ok(())
    }
    fn encode_map(&mut self, x: &Map) -> EncodeResult {
        try!(self.writer.write_u8(MAP_EXT));
        try!(self.writer.write_u32::<BigEndian>(x.entries.len() as u32));
        for &(ref k, ref v) in &x.entries {
            try!(self.encode_term(k));
            try!(self.encode_term(v));
        }
        Ok(())
    }
    fn encode_binary(&mut self, x: &Binary) -> EncodeResult {
        try!(self.writer.write_u8(BINARY_EXT));
        try!(self.writer.write_u32::<BigEndian>(x.bytes.len() as u32));
        try!(self.writer.write_all(&x.bytes));
        Ok(())
    }
    fn encode_bit_binary(&mut self, x: &BitBinary) -> EncodeResult {
        try!(self.writer.write_u8(BIT_BINARY_EXT));
        try!(self.writer.write_u32::<BigEndian>(x.bytes.len() as u32));
        try!(self.writer.write_u8(x.tail_bits_size));
        if !x.bytes.is_empty() {
            try!(self.writer.write_all(&x.bytes[0..x.bytes.len() - 1]));
            try!(self.writer.write_u8(x.bytes[x.bytes.len() - 1] << (8 - x.tail_bits_size)));
        }
        Ok(())
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
        if 0 <= x.value && x.value <= std::u8::MAX as i32 {
            try!(self.writer.write_u8(SMALL_INTEGER_EXT));
            try!(self.writer.write_u8(x.value as u8));
        } else {
            try!(self.writer.write_u8(INTEGER_EXT));
            try!(self.writer.write_i32::<BigEndian>(x.value as i32));
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
    fn encode_reference(&mut self, x: &Reference) -> EncodeResult {
        try!(self.writer.write_u8(NEW_REFERENCE_EXT));
        if x.id.len() > std::u16::MAX as usize {
            return aux::invalid_data_error(format!("Too large ID: {}*4 bytes", x.id.len()));
        }
        try!(self.writer.write_u16::<BigEndian>(x.id.len() as u16));
        try!(self.encode_atom(&x.node));
        try!(self.writer.write_u8(x.creation));
        for n in &x.id {
            try!(self.writer.write_u32::<BigEndian>(*n));
        }
        Ok(())
    }
    fn encode_external_fun(&mut self, x: &ExternalFun) -> EncodeResult {
        try!(self.writer.write_u8(EXPORT_EXT));
        try!(self.encode_atom(&x.module));
        try!(self.encode_atom(&x.function));
        try!(self.encode_fix_integer(&FixInteger::from(x.arity as i32)));
        Ok(())
    }
    fn encode_internal_fun(&mut self, x: &InternalFun) -> EncodeResult {
        match *x {
            InternalFun::Old { ref module, ref pid, ref free_vars, index, uniq } => {
                try!(self.writer.write_u8(FUN_EXT));
                try!(self.writer.write_u32::<BigEndian>(free_vars.len() as u32));
                try!(self.encode_pid(pid));
                try!(self.encode_atom(module));
                try!(self.encode_fix_integer(&FixInteger::from(index)));
                try!(self.encode_fix_integer(&FixInteger::from(uniq)));
                for v in free_vars {
                    try!(self.encode_term(v));
                }
            }
            InternalFun::New { ref module,
                               arity,
                               ref pid,
                               ref free_vars,
                               index,
                               ref uniq,
                               old_index,
                               old_uniq } => {
                try!(self.writer.write_u8(NEW_FUN_EXT));

                let mut buf = Vec::new();
                {
                    let mut tmp = Encoder::new(&mut buf);
                    try!(tmp.writer.write_u8(arity));
                    try!(tmp.writer.write_all(uniq));
                    try!(tmp.writer.write_u32::<BigEndian>(index));
                    try!(tmp.writer.write_u32::<BigEndian>(free_vars.len() as u32));
                    try!(tmp.encode_atom(module));
                    try!(tmp.encode_fix_integer(&FixInteger::from(old_index)));
                    try!(tmp.encode_fix_integer(&FixInteger::from(old_uniq)));
                    try!(tmp.encode_pid(pid));
                    for v in free_vars {
                        try!(tmp.encode_term(v));
                    }
                }
                try!(self.writer.write_u32::<BigEndian>(4 + buf.len() as u32));
                try!(self.writer.write_all(&buf));
            }
        }
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
