/// Copyright (c) 2015, Takeru Ohta <phjgt308@gmail.com>
///
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Term {
    Int (i32),
    Float (f64),
    Atom (String),
}

const VERSION: u8 = 131;

const TAG_SMALL_INTEGER: u8 = 97; // 10.4: SMALL_INTEGER_EXT
const TAG_INTEGER: u8 = 98; // 10.5: INTEGER_ET
const TAG_FLOAT: u8 = 99; // 10.6: FLOAT_EXT
const TAG_ATOM: u8 = 100; // 10.7: ATOM_EXT
const TAG_NEW_FLOAT: u8 = 70; // 10.26: NEW_FLOAT_EXT

pub fn encode() {
    unimplemented!()
}

// TODO: returns remaining bytes
pub fn decode(bytes: &[u8]) -> Option<Term> {
    if bytes.len() < 2 { return None }
    if bytes[0] != VERSION { return None }

    let tag = bytes[1];
    match tag {
        TAG_SMALL_INTEGER => decode_small_integer(&bytes[2..]),
        TAG_INTEGER => decode_integer(&bytes[2..]),
        TAG_FLOAT => decode_float(&bytes[2..]),
        TAG_ATOM => decode_atom(&bytes[2..]),
        TAG_NEW_FLOAT => decode_new_float(&bytes[2..]),
        _ => unimplemented!(),
    }
}

fn decode_float(bytes: &[u8]) -> Option<Term> {
    if bytes.len() < 31 { return None }

    unimplemented!()
    // std::str::from_utf8(bytes).ok()
    //     .and_then(|s| s.parse::<f32>().ok()) // TODO: Supports scientific notations
    //     .and_then(|f| Some(Term::Float(f)))
}

fn decode_atom(bytes: &[u8]) -> Option<Term> {
    if bytes.len() < 2 { return None }

    let len = to_u16(bytes) as usize;
    if len > 255 { return None }

    let bytes = &bytes[2..];
    if bytes.len() < len { return None }

    std::str::from_utf8(&bytes[..len]).ok().and_then(|s| Some(Term::Atom(s.to_string())) )
}

fn decode_new_float(bytes: &[u8]) -> Option<Term> {
    if bytes.len() < 8 { return None }

    unimplemented!()
    // Some(Term::Float(to_u64(bytes) as f64))
}

fn decode_small_integer(bytes: &[u8]) -> Option<Term> {
    if bytes.len() < 1 { return None }

    Some(Term::Int(bytes[0] as i32))
}

fn decode_integer(bytes: &[u8]) -> Option<Term> {
    // TODO: finds a standard libary function
    if bytes.len() < 4 { return None }

    let n = to_u32(bytes);
    Some(Term::Int(n as i32))
}

fn to_u16(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) + (bytes[1] as u16)
}

fn to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24) + ((bytes[1] as u32) << 16) + ((bytes[2] as u32) << 8) + (bytes[3] as u32)
}

// fn to_u64(bytes: &[u8]) -> u64 {
//     ((to_u32(bytes) as u64) << 32) + (to_u32(&bytes[4..]) as u64)
// }
