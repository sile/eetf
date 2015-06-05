/// Copyright (c) 2015, Takeru Ohta <phjgt308@gmail.com>
///
extern crate eetf;

use eetf::*;

#[test]
fn decode_empty() {
    let input = [];
    let expected = None;
    assert_eq!(expected, decode(&input));
}

#[test]
fn decode_small_integer() {
    let input = [131,97,5];
    let expected = 5;
    assert_eq!(Term::Int(expected), decode(&input).unwrap());
}

#[test]
fn decode_integer() {
    let input = [131,98,0,0,4,210];
    let expected = 1234;
    assert_eq!(Term::Int(expected), decode(&input).unwrap());

    let input = [131,98,255,255,251,46];
    let expected = -1234;
    assert_eq!(Term::Int(expected), decode(&input).unwrap());
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

#[test]
fn decode_atom() {
    let input = [131,100,0,4,104,111,103,101];
    let expected = "hoge".to_string();
    assert_eq!(Term::Atom(expected), decode(&input).unwrap());
}

#[test]
fn decode_nil() {
    let input = [131,106];
    let expected = vec![];
    assert_eq!(Term::List(expected), decode(&input).unwrap());
}
