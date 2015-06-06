/// Copyright (c) 2015, Takeru Ohta <phjgt308@gmail.com>
//
extern crate eetf;
extern crate num;

use eetf::*;
use std::io::Cursor;
use std::collections::BTreeMap;
use num::bigint::ToBigInt;

macro_rules! assert_decode {
    ($x:expr, $y:expr) => {{
        let mut cur: Cursor<&[u8]> = Cursor::new(&$y);
        assert_eq!($x, decode(&mut cur))
    }}
}

#[test]
fn decode_empty() {
    let input = [];
    let expected = None;
    assert_decode!(expected, input);
}

#[test]
fn decode_atom_cache_ref() {
    let input = [131,82,5];
    let expected = 5;
    assert_decode!(Some(Term::AtomCacheRef(expected)), input);
}

#[test]
fn decode_float() {
    // let input = [131,99,45,49,46,50,51,52,53,48,48,48,48,48,48,48,48,48,48,48,48,50,56,52,50,101,43,48,50,0,0,0,0];
    // let expected = FloatWrap{value: -123.45};
    // assert_decode!(Some(Term::Float(expected)), input);

    let input = [131,99,49,46,50,51,52,53,48,48,48,48,48,48,48,48,48,48,48,48,48,55,48,53,101,45,48,50,0,0,0,0,0];
    let expected = FloatWrap{value: 0.012345};
    assert_decode!(Some(Term::Float(expected)), input);
}
#[test]
fn decode_new_float() {
    let input = [131,70,63,137,72,85,218,39,40,99];
    let expected = FloatWrap{value: 0.012345};
    assert_decode!(Some(Term::Float(expected)), input);
}

#[test]
fn decode_small_integer() {
    let input = [131,97,5];
    let expected = 5;
    assert_decode!(Some(Term::Int(expected)), input);
}

#[test]
fn decode_small_big() {
    let input = [131,110,1,0,10];
    let expected = Term::BigInt(10.to_bigint().unwrap());
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_large_big() {
    let input = [131,111,0,0,0,1,1,10];
    let expected = Term::BigInt(-10.to_bigint().unwrap());
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_integer() {
    let input = [131,98,0,0,4,210];
    let expected = 1234;
    assert_decode!(Some(Term::Int(expected)), input);

    let input = [131,98,255,255,251,46];
    let expected = -1234;
    assert_decode!(Some(Term::Int(expected)), input);
}

#[test]
fn decode_atom() {
    let input = [131,100,0,4,104,111,103,101];
    let expected = "hoge".to_string();
    assert_decode!(Some(Term::Atom(expected)), input);
}

#[test]
fn decode_small_atom() {
    let input = [131,115,4,104,111,103,101];
    let expected = "hoge".to_string();
    assert_decode!(Some(Term::Atom(expected)), input);
}

#[test]
fn decode_atom_utf8() {
    let input = [131,118,0,4,104,111,103,101];
    let expected = "hoge".to_string();
    assert_decode!(Some(Term::Atom(expected)), input);
}

#[test]
fn decode_small_atom_utf8() {
    let input = [131,119,4,104,111,103,101];
    let expected = "hoge".to_string();
    assert_decode!(Some(Term::Atom(expected)), input);
}

#[test]
fn decode_small_tuple() {
    let input = [131,104,2,97,1,100,0,3,111,110,101];
    let expected = vec![Term::Int(1), Term::Atom("one".to_string())];
    assert_decode!(Some(Term::Tuple(expected)), input);
}

#[test]
fn decode_large_tuple() {
    let input = [131,105,0,0,0,2,97,1,100,0,3,111,110,101];
    let expected = vec![Term::Int(1), Term::Atom("one".to_string())];
    assert_decode!(Some(Term::Tuple(expected)), input);
}

#[test]
fn decode_nil() {
    let input = [131,106];
    let expected = vec![];
    assert_decode!(Some(Term::List(expected)), input);
}

#[test]
fn decode_string() {
    let input = [131,107,0,4,104,111,103,101];
    let expected = vec![104,111,103,101].iter().map(|x| Term::Int(*x)).collect(); // "hoge"
    assert_decode!(Some(Term::List(expected)), input);
}

#[test]
fn decode_list() {
    let input = [131,108,0,0,0,2,97,1,100,0,3,111,110,101,106];
    let expected = vec![Term::Int(1), Term::Atom("one".to_string())];
    assert_decode!(Some(Term::List(expected)), input);
}

#[test]
fn decode_improper_list() {
    let input = [131,108,0,0,0,1,97,1,100,0,3,111,110,101];
    let expected = Term::ImproperList(vec![Term::Int(1)], Box::new(Term::Atom("one".to_string())));
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_binary() {
    let input = [131,109,0,0,0,4,104,111,103,101];
    let expected = vec![104,111,103,101]; // "hoge"
    assert_decode!(Some(Term::Binary(expected)), input);
}

#[test]
fn decode_bit_binary() {
    let input = [131,77,0,0,0,4,6,1,2,3,16];
    let expected = Term::BitStr(vec![1,2,3,16], 6);
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_reference() {
    let input = [131,101,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,49,0];
    let expected = Term::Ref("nonode@nohost".to_string(), vec![49], 0);
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_new_reference() {
    let input = [131,114,0,3,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,0,123,0,0,0,0,0,0,0,0];
    let expected = Term::Ref("nonode@nohost".to_string(), vec![123,0,0], 0);
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_port() {
    let input = [131,102,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,49,0];
    let expected = Term::Port("nonode@nohost".to_string(), 49, 0);
    assert_decode!(Some(expected), input);
}


#[test]
fn decode_pid() {
    let input = [131,103,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,40,0,0,0,0,0];
    let expected = Term::Pid("nonode@nohost".to_string(), 40, 0, 0);
    assert_decode!(Some(expected), input);
}

#[test]
fn decode_map() {
    let input = [131,116,0,0,0,2,97,1,97,2,100,0,3,111,110,101,100,0,3,116,119,111];
    let mut expected = BTreeMap::new();
    expected.insert(Term::Int(1), Term::Int(2));
    expected.insert(Term::Atom("one".to_string()), Term::Atom("two".to_string()));
    assert_decode!(Some(Term::Map(expected)), input);
}

// #[test]
// fn decode_float() {
//     let input = [131,99,49,46,50,51,51,57,57,57,57,57,57,57,57,57,57,57,57,57,56,53,55,57,101,43,48,49,0,0,0,0,0];
//     let expected = 12.34;
//     assert_eq!(Term::Float(expected), decode(&input).unwrap());
// }

// #[test]
// fn decode_new_float() {
//     let input = [131,70,64,40,174,20,122,225,71,174];
//     let expected = 12.34;
//     assert_eq!(Term::Float(expected), decode(&input).unwrap());
// }
