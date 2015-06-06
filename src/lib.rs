/// Copyright (c) 2015, Takeru Ohta <phjgt308@gmail.com>
///
use std::io::Read;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Term {
    Int (i32),
    Float (f64),
    Atom (String),
    List (Vec<Term>),
    ImproperList (Vec<Term>, Box<Term>),
    Tuple (Vec<Term>),
    Binary (Vec<u8>),
}

const VERSION: u8 = 131;

const TAG_ATOM_CACHE_REF:  u8 = 82;  // 10.3: ATOM_CACHE_REF
const TAG_SMALL_INTEGER:   u8 = 97;  // 10.4: SMALL_INTEGER_EXT
const TAG_INTEGER:         u8 = 98;  // 10.5: INTEGER_EXT
const TAG_FLOAT:           u8 = 99;  // 10.6: FLOAT_EXT
const TAG_ATOM:            u8 = 100; // 10.7: ATOM_EXT
const TAG_REFERENCE:       u8 = 101; // 10.8: REFERENCE_EXT
const TAG_PORT:            u8 = 102; // 10.9: PORT_EXT
const TAG_PID:             u8 = 103; // 10.10: PID_EXT
const TAG_SMALL_TUPLE:     u8 = 104; // 10.11: SMALL_TUPLE_EXT
const TAG_LARGE_TUPLE:     u8 = 105; // 10.12: LARGE_TUPLE_EXT
const TAG_MAP:             u8 = 116; // 10.13: MAP_EXT
const TAG_NIL:             u8 = 106; // 10.14: NIL_EXT
const TAG_STRING:          u8 = 107; // 10.15: STRING_EXT
const TAG_LIST:            u8 = 108; // 10.16: LIST_EXT
const TAG_BINARY:          u8 = 109; // 10.17: BINARY_EXT
const TAG_SMALL_BIG:       u8 = 110; // 10.18: SMALL_BIG_EXT
const TAG_LARGE_BIG:       u8 = 111; // 10.19: LARGE_BIG_EXT
const TAG_NEW_REFERENCE:   u8 = 114; // 10.20: NEW_REFERENCE_EXT
const TAG_SMALL_ATOM:      u8 = 115; // 10.21: SMALL_ATOM_EXT
const TAG_FUN:             u8 = 117; // 10.22: FUN_EXT
const TAG_NEW_FUN:         u8 = 112; // 10.23: NEW_FUN_EXT
const TAG_EXPORT:          u8 = 113; // 10.24: EXPORT_EXT
const TAG_BIT_BINARY:      u8 = 77;  // 10.25: BIT_BINARY_EXT
const TAG_NEW_FLOAT:       u8 = 70;  // 10.26: NEW_FLOAT_EXT
const TAG_ATOM_UTF8:       u8 = 118; // 10.27: ATOM_UTF8_EXT
const TAG_SMALL_ATOM_UTF8: u8 = 119; // 10.28: SMALL_ATOM_UTF8_EXT

pub fn encode() {
    unimplemented!()
}

pub fn decode<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|version| {
        if version != VERSION { return None }
        decode_term(r)
    })
}

fn decode_term<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|tag| {
        match tag {
            TAG_ATOM_CACHE_REF  => unimplemented!(),
            TAG_SMALL_INTEGER   => decode_small_integer(r),
            TAG_INTEGER         => decode_integer(r),
            TAG_FLOAT           => unimplemented!(),
            TAG_ATOM            => decode_atom(r),
            TAG_REFERENCE       => unimplemented!(),
            TAG_PORT            => unimplemented!(),
            TAG_PID             => unimplemented!(),
            TAG_SMALL_TUPLE     => decode_small_tuple(r),
            TAG_LARGE_TUPLE     => decode_large_tuple(r),
            TAG_MAP             => unimplemented!(),
            TAG_NIL             => decode_nil(r),
            TAG_STRING          => decode_string(r),
            TAG_LIST            => decode_list(r),
            TAG_BINARY          => decode_binary(r),
            TAG_SMALL_BIG       => unimplemented!(),
            TAG_LARGE_BIG       => unimplemented!(),
            TAG_NEW_REFERENCE   => unimplemented!(),
            TAG_SMALL_ATOM      => decode_small_atom(r),
            TAG_FUN             => unimplemented!(),
            TAG_NEW_FUN         => unimplemented!(),
            TAG_EXPORT          => unimplemented!(),
            TAG_BIT_BINARY      => unimplemented!(),
            TAG_NEW_FLOAT       => unimplemented!(),
            TAG_ATOM_UTF8       => decode_atom_utf8(r),
            TAG_SMALL_ATOM_UTF8 => decode_small_atom_utf8(r),
            _                   => panic!("Unknown tag: {}", tag)
        }
    })
}

fn decode_small_tuple<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|arity| {
        let mut tuple = Vec::with_capacity(arity as usize);
        for _ in 0..arity {
            match decode_term(r) {
                None    => return None,
                Some(v) => tuple.push(v),
            }
        }
        Some(Term::Tuple(tuple))
    })
}

fn decode_large_tuple<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|arity| {
        let mut tuple = Vec::with_capacity(arity as usize);
        for _ in 0..arity {
            match decode_term(r) {
                None    => return None,
                Some(v) => tuple.push(v),
            }
        }
        Some(Term::Tuple(tuple))
    })
}

fn decode_nil<T: Read>(_r: &mut T) -> Option<Term> {
    Some(Term::List(vec![]))
}

fn decode_string<T: Read>(r: &mut T) -> Option<Term> {
    read_u16(r).and_then(|count| {
        let mut buf = vec![0; count as usize];
        if ! read_full(r, &mut buf) { return None }
        Some(Term::List(buf.iter().map(|x| Term::Int(*x as i32)).collect()))
    })
}

fn decode_list<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|count| {
        let mut elements = Vec::with_capacity(count as usize);
        for _ in 0..count {
            match decode_term(r) {
                None    => return None,
                Some(v) => elements.push(v),
            }
        }
        match decode_term(r) {
            None                                    => None,
            Some(Term::List(ref l)) if l.is_empty() => Some(Term::List(elements)), // XXX
            Some(v)                                 => Some(Term::ImproperList(elements, Box::new(v))),
        }
    })
}

fn decode_atom<T: Read>(r: &mut T) -> Option<Term> {
    read_u16(r).and_then(|len| {
        if len > 255 { panic!("Too large atom length: {}", len) }

        let mut buf = &mut [0; 255][..(len as usize)];
        if ! read_full(r, buf) { return None }

        std::str::from_utf8(buf).ok().and_then(|s| Some(Term::Atom(s.to_string())) )
    })
}

fn decode_small_atom<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|len| {
        let mut buf = &mut [0; 255][..(len as usize)];
        if ! read_full(r, buf) { return None }

        std::str::from_utf8(buf).ok().and_then(|s| Some(Term::Atom(s.to_string())) )
    })
}

fn decode_atom_utf8<T: Read>(r: &mut T) -> Option<Term> {
    read_u16(r).and_then(|len| {
        let mut buf: Vec<u8> = vec![0; len as usize];
        if ! read_full(r, &mut buf[..]) { return None }

        std::str::from_utf8(&buf[..]).ok().and_then(|s| Some(Term::Atom(s.to_string())) )
    })
}

fn decode_small_atom_utf8<T: Read>(r: &mut T) -> Option<Term> {
    decode_small_atom(r) // XXX:
}

fn decode_binary<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|len| {
        let mut buf = vec![0; len as usize];
        if ! read_full(r, &mut buf) { return None }
        Some(Term::Binary(buf))
    })
}

fn decode_small_integer<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|n| Some(Term::Int(n as i32)))
}

fn decode_integer<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|n| Some(Term::Int(n as i32)))
}

fn read_u8<T: Read>(r: &mut T) -> Option<u8> {
    let mut buf: [u8; 1] = [0; 1];
    if read_full(r, &mut buf) { Some(buf[0]) } else { None }
}

fn read_u16<T: Read>(r: &mut T) -> Option<u16> {
    let mut buf: [u8; 2] = [0; 2];
    if !read_full(r, &mut buf) { return None }
    Some(((buf[0] as u16) << 8) + (buf[1] as u16))
}

fn read_u32<T: Read>(r: &mut T) -> Option<u32> {
    let mut buf: [u8; 4] = [0; 4];
    if !read_full(r, &mut buf) { return None }
    Some(((buf[0] as u32) << 24) + ((buf[1] as u32) << 16) + ((buf[2] as u32) << 8) + (buf[3] as u32))
}

fn read_full<T: Read>(r: &mut T, buf: &mut [u8]) -> bool {
    match r.read(buf).ok() {
        None    => false,
        Some(n) => n == buf.len(),
    }
}
