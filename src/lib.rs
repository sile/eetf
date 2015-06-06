/// Copyright (c) 2015, Takeru Ohta <phjgt308@gmail.com>
///
extern crate num;

use std::io::Read;
use std::cmp::Eq;
use std::cmp::{Ord,Ordering};
use std::collections::BTreeMap;
use num::bigint::{BigInt,Sign};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
pub enum Term { // TODO: Erlangのtermの比較順序に従う
    Int (i32),
    BigInt (BigInt),
    Float (FloatWrap),
    Atom (String),
    AtomCacheRef (u8),
    List (Vec<Term>),
    ImproperList (Vec<Term>, Box<Term>),
    Tuple (Vec<Term>),
    Binary (Vec<u8>),
    BitStr (Vec<u8>, u8), // TODO: Replaces to a native bitstring type
    Ref (String, Vec<u32>, u8),
    Port (String, u32, u8),
    Pid (String, u32, u32, u8),
    Map (BTreeMap<Term, Term>),
}

#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Debug)]
pub struct FloatWrap{pub value: f64}

impl Eq for FloatWrap {}

impl Ord for FloatWrap {
    fn cmp(&self, other: &FloatWrap) -> Ordering {
        // XXX: Maybe inaccurate
        if self.value < other.value { return Ordering::Less }
        if self.value > other.value { return Ordering::Greater }
        return Ordering::Equal
    }
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
            TAG_ATOM_CACHE_REF  => decode_atom_cache_ref(r),
            TAG_SMALL_INTEGER   => decode_small_integer(r),
            TAG_INTEGER         => decode_integer(r),
            TAG_FLOAT           => decode_float(r),
            TAG_ATOM            => decode_atom(r),
            TAG_REFERENCE       => decode_reference(r),
            TAG_PORT            => decode_port(r),
            TAG_PID             => decode_pid(r),
            TAG_SMALL_TUPLE     => decode_small_tuple(r),
            TAG_LARGE_TUPLE     => decode_large_tuple(r),
            TAG_MAP             => decode_map(r),
            TAG_NIL             => decode_nil(r),
            TAG_STRING          => decode_string(r),
            TAG_LIST            => decode_list(r),
            TAG_BINARY          => decode_binary(r),
            TAG_SMALL_BIG       => decode_small_big(r),
            TAG_LARGE_BIG       => decode_large_big(r),
            TAG_NEW_REFERENCE   => decode_new_reference(r),
            TAG_SMALL_ATOM      => decode_small_atom(r),
            TAG_FUN             => unimplemented!(),
            TAG_NEW_FUN         => unimplemented!(),
            TAG_EXPORT          => unimplemented!(),
            TAG_BIT_BINARY      => decode_bit_binary(r),
            TAG_NEW_FLOAT       => decode_new_float(r),
            TAG_ATOM_UTF8       => decode_atom_utf8(r),
            TAG_SMALL_ATOM_UTF8 => decode_small_atom_utf8(r),
            _                   => panic!("Unknown tag: {}", tag)
        }
    })
}

fn decode_atom_cache_ref<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|index| Some(Term::AtomCacheRef(index)))
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

fn decode_map<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|arity| {
        let mut map = BTreeMap::new();
        for _ in 0..arity {
            match decode_term(r).and_then(|k| decode_term(r).and_then(|v| Some(map.insert(k, v)))) {
                None => return None,
                rlt  => rlt,
            };
        };
        Some(Term::Map(map))
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

fn decode_reference<T: Read>(r: &mut T) -> Option<Term> {
    decode_term(r).and_then(|t| {
        match t {
            Term::Atom(node) =>
                read_u32(r).and_then(|id| read_u8(r).and_then(|creation| {
                    Some(Term::Ref(node, vec![id], creation))
                })),
            _ =>
                panic!("Unexpected term: {:?} (an atom is expected)", t),
        }
    })
}

fn decode_new_reference<T: Read>(r: &mut T) -> Option<Term> {
    read_u16(r).and_then(|len| decode_term(r).and_then(|t| {
        match t {
            Term::Atom(node) =>
                read_u8(r).and_then(|creation| {
                    let mut ids = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        match read_u32(r) {
                            None     => return None,
                            Some(id) => ids.push(id),
                        }
                    };
                    Some(Term::Ref(node, ids, creation))
                }),
            _ =>
                panic!("Unexpected term: {:?} (an atom is expected)", t),
        }
    }))
}

fn decode_port<T: Read>(r: &mut T) -> Option<Term> {
    decode_term(r).and_then(|t| {
        match t {
            Term::Atom(node) =>
                read_u32(r).and_then(|id| read_u8(r).and_then(|creation| {
                    Some(Term::Port(node, id, creation))
                })),
            _ =>
                panic!("Unexpected term: {:?} (an atom is expected)", t),
        }
    })
}

fn decode_pid<T: Read>(r: &mut T) -> Option<Term> {
    decode_term(r).and_then(|t| {
        match t {
            Term::Atom(node) =>
                read_u32(r).and_then(|id| read_u32(r).and_then(|serial| read_u8(r).and_then(|creation| {
                    Some(Term::Pid(node, id, serial, creation))
                }))),
            _ =>
                panic!("Unexpected term: {:?} (an atom is expected)", t),
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

fn decode_bit_binary<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|len| read_u8(r).and_then(|bits| {
        if bits == 0 || bits > 7 { panic!("Wrong bits: {}", bits) }

        let mut buf = vec![0; len as usize];
        if ! read_full(r, &mut buf) { return None }
        Some(Term::BitStr(buf, bits))
    }))
}

fn decode_large_big<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|count| read_u8(r).and_then(|sign| {
        let sign =
            match sign {
                0 => Sign::Plus,
                1 => Sign::Minus,
                _ => return None,
            };
        let mut bytes = vec![0; count as usize];
        if ! read_full(r, &mut bytes) { return None }
        bytes.reverse(); // LE to BE
        Some(Term::BigInt(BigInt::from_bytes_be(sign, &bytes)))
    }))
}

fn decode_small_big<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|count| read_u8(r).and_then(|sign| {
        let sign =
            match sign {
                0 => Sign::Plus,
                1 => Sign::Minus,
                _ => return None,
            };
        let mut bytes = vec![0; count as usize];
        if ! read_full(r, &mut bytes) { return None }
        bytes.reverse(); // LE to BE
        Some(Term::BigInt(BigInt::from_bytes_be(sign, &bytes)))
    }))
}

fn decode_small_integer<T: Read>(r: &mut T) -> Option<Term> {
    read_u8(r).and_then(|n| Some(Term::Int(n as i32)))
}

fn decode_integer<T: Read>(r: &mut T) -> Option<Term> {
    read_u32(r).and_then(|n| Some(Term::Int(n as i32)))
}

fn decode_float<T: Read>(r: &mut T) -> Option<Term> {
    let mut buf: [u8; 31] = [0; 31];
    if !read_full(r, &mut buf) { return None }
    std::str::from_utf8(&buf).ok().and_then(|s| {
        let s = s.trim_right_matches('\u{0}');
        let v: Vec<&str> = s.splitn(2, 'e').collect();
        if v.len() != 2 { return None }

        v[0].parse().ok().and_then(|decimal: f64| v[1].trim_left_matches('+').parse().ok().and_then(|exp| {
            let f = decimal * 10f64.powi(exp);
            Some(Term::Float(FloatWrap{value: f}))
        }))
    })
}

fn decode_new_float<T: Read>(r: &mut T) -> Option<Term> {
    use std::mem;
    read_u64(r).and_then(|n| {
        unsafe {
            Some(Term::Float(FloatWrap{value: mem::transmute::<u64, f64>(n)}))
        }
    })
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

fn read_u64<T: Read>(r: &mut T) -> Option<u64> {
    read_u32(r).and_then(|high| read_u32(r).and_then(|low| Some(((high as u64) << 32) + (low as u64))))
}

fn read_full<T: Read>(r: &mut T, buf: &mut [u8]) -> bool {
    match r.read(buf).ok() {
        None    => false,
        Some(n) => n == buf.len(),
    }
}
