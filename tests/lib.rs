extern crate eetf;
extern crate num;

use std::io::Cursor;
use eetf::*;

#[test]
fn atom_test() {
    // Display
    assert_eq!("'foo'", Atom::from("foo").to_string());
    assert_eq!(r#"'fo\'o'"#, Atom::from(r#"fo'o"#).to_string());
    assert_eq!(r#"'fo\\o'"#, Atom::from(r#"fo\o"#).to_string());

    // Decode
    assert_eq!(Ok(Atom::from("foo")),
               decode(&[131, 100, 0, 3, 102, 111, 111]).into_atom()); // ATOM_EXT
    assert_eq!(Ok(Atom::from("foo")),
               decode(&[131, 115, 3, 102, 111, 111]).into_atom()); // SMALL_ATOM_EXT
    assert_eq!(Ok(Atom::from("foo")),
               decode(&[131, 118, 0, 3, 102, 111, 111]).into_atom()); // ATOM_UTF8_EXT
    assert_eq!(Ok(Atom::from("foo")),
               decode(&[131, 119, 3, 102, 111, 111]).into_atom()); // SMALL_ATOM_UTF8_EXT

    // Encode
    assert_eq!(vec![131, 100, 0, 3, 102, 111, 111],
               encode(Term::from(Atom::from("foo"))));
}

#[test]
fn integer_test() {
    // Display
    assert_eq!("123", FixInteger::from(123).to_string());
    assert_eq!("123", BigInteger::from(123).to_string());
    assert_eq!("-123", FixInteger::from(-123).to_string());
    assert_eq!("-123", BigInteger::from(-123).to_string());

    // Decode
    assert_eq!(Ok(FixInteger::from(10)),
               decode(&[131, 97, 10]).into_fix_integer()); // SMALL_INTEGER_EXT
    assert_eq!(Ok(FixInteger::from(1000)),
               decode(&[131, 98, 0, 0, 3, 232]).into_fix_integer()); // INTEGER_EXT
    assert_eq!(Ok(FixInteger::from(-1000)),
               decode(&[131, 98, 255, 255, 252, 24]).into_fix_integer()); // INTEGER_EXT
    assert_eq!(Ok(BigInteger::from(0)),
               decode(&[131, 110, 1, 0, 0]).into_big_integer()); // SMALL_BIG_EXT
    assert_eq!(Ok(BigInteger::from(513)),
               decode(&[131, 110, 2, 0, 1, 2]).into_big_integer()); // SMALL_BIG_EXT
    assert_eq!(Ok(BigInteger::from(-513)),
               decode(&[131, 110, 2, 1, 1, 2]).into_big_integer()); // SMALL_BIG_EXT
    assert_eq!(Ok(BigInteger::from(513)),
               decode(&[131, 111, 0, 0, 0, 2, 0, 1, 2]).into_big_integer()); // LARGE_BIG_EXT

    // Encode
    assert_eq!(vec![131, 97, 0], encode(Term::from(FixInteger::from(0))));
    assert_eq!(vec![131, 98, 255, 255, 255, 255],
               encode(Term::from(FixInteger::from(-1))));
    assert_eq!(vec![131, 98, 0, 0, 3, 232],
               encode(Term::from(FixInteger::from(1000))));
    assert_eq!(vec![131, 110, 1, 0, 0],
               encode(Term::from(BigInteger::from(0))));
    assert_eq!(vec![131, 110, 1, 1, 10],
               encode(Term::from(BigInteger::from(-10))));
    assert_eq!(vec![131, 110, 5, 0, 0, 228, 11, 84, 2],
               encode(Term::from(BigInteger::from(10000000000))));
}

#[test]
fn float_test() {
    // Display
    assert_eq!("123", Float::from(123.0).to_string());
    assert_eq!("123.4", Float::from(123.4).to_string());
    assert_eq!("-123.4", Float::from(-123.4).to_string());

    // Decode
    assert_eq!(Ok(Float::from(123.456)),
               // NEW_FLOAT_EXT
               decode(&[131, 70, 64, 94, 221, 47, 26, 159, 190, 119]).into_float());
    assert_eq!(Ok(Float::from(-123.456)),
               // NEW_FLOAT_EXT
               decode(&[131, 70, 192, 94, 221, 47, 26, 159, 190, 119]).into_float());
    // Encode
    assert_eq!(vec![131, 70, 64, 94, 221, 47, 26, 159, 190, 119],
               encode(Term::from(Float::from(123.456))));
}

#[test]
fn pid_test() {
    // Display
    assert_eq!(r#"<'nonode@nohost'.1.2>"#,
               Pid::from(("nonode@nohost", 1, 2)).to_string());

    // Decode
    assert_eq!(Ok(Pid::from(("nonode@nohost", 49, 0))),
               decode(&[131, 103, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104,
                        111, 115, 116, 0, 0, 0, 49, 0, 0, 0, 0, 0])
                   .into_pid()); // PID_EXT

    // Encode
    assert_eq!(vec![131, 103, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111,
                    115, 116, 0, 0, 0, 49, 0, 0, 0, 0, 0],
               encode(Term::from(Pid::from(("nonode@nohost", 49, 0)))));
}

#[test]
fn port_test() {
    // Display
    assert_eq!(r#"#Port<'nonode@nohost'.1>"#,
               Port::from(("nonode@nohost", 1)).to_string());

    // Decode
    assert_eq!(Ok(Port::from(("nonode@nohost", 366))),
               decode(&[131, 102, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104,
                        111, 115, 116, 0, 0, 1, 110, 0])
                   .into_port()); // PORT_EXT

    // Encode
    assert_eq!(vec![131, 102, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111,
                    115, 116, 0, 0, 1, 110, 0],
               encode(Term::from(Port::from(("nonode@nohost", 366)))));
}

#[test]
fn reference_test() {
    // Display
    assert_eq!(r#"#Ref<'nonode@nohost'.1>"#,
               Reference::from(("nonode@nohost", 1)).to_string());

    // Decode
    assert_eq!(Ok(Reference::from(("nonode@nohost", vec![138016, 262145, 0]))),
               decode(&[131, 114, 0, 3, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111,
                        104, 111, 115, 116, 0, 0, 2, 27, 32, 0, 4, 0, 1, 0, 0, 0, 0])
                   .into_reference()); // NEW_REFERENCE_EXT
    assert_eq!(Ok(Reference::from(("foo", vec![2]))),
               // NEW_REFERENCE_EXT
               decode(&[131, 101, 115, 3, 102, 111, 111, 0, 0, 0, 2, 0]).into_reference());

    // Encode
    assert_eq!(vec![131, 114, 0, 1, 100, 0, 3, 102, 111, 111, 0, 0, 0, 0, 123],
               encode(Term::from(Reference::from(("foo", 123)))));
}

#[test]
fn external_fun_test() {
    // Display
    assert_eq!(r#"fun 'foo':'bar'/3"#,
               ExternalFun::from(("foo", "bar", 3)).to_string());

    // Decode
    assert_eq!(Ok(ExternalFun::from(("foo", "bar", 3))),
               decode(&[131, 113, 100, 0, 3, 102, 111, 111, 100, 0, 3, 98, 97, 114, 97, 3])
                   .into_external_fun());

    // Encode
    assert_eq!(vec![131, 113, 100, 0, 3, 102, 111, 111, 100, 0, 3, 98, 97, 114, 97, 3],
               encode(Term::from(ExternalFun::from(("foo", "bar", 3)))));
}

#[test]
fn internal_fun_test() {
    let term = InternalFun::New {
        module: Atom::from("a"),
        arity: 1,
        pid: Pid::from(("nonode@nohost", 36, 0)),
        index: 0,
        uniq: [115, 60, 203, 97, 151, 228, 98, 75, 71, 169, 49, 166, 34, 126, 65, 11],
        old_index: 0,
        old_uniq: 60417627,
        free_vars: vec![Term::from(FixInteger::from(10))],
    };
    let bytes = [131, 112, 0, 0, 0, 68, 1, 115, 60, 203, 97, 151, 228, 98, 75, 71, 169, 49, 166,
                 34, 126, 65, 11, 0, 0, 0, 0, 0, 0, 0, 1, 100, 0, 1, 97, 97, 0, 98, 3, 153, 230,
                 91, 103, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115,
                 116, 0, 0, 0, 36, 0, 0, 0, 0, 0, 97, 10];
    // Decode
    assert_eq!(Ok(term.clone()), decode(&bytes).into_internal_fun());

    // Encode
    assert_eq!(Vec::from(&bytes[..]), encode(Term::from(term)));
}

#[test]
fn binary_test() {
    // Display
    assert_eq!("<<1,2,3>>", Binary::from(vec![1, 2, 3]).to_string());

    // Decode
    assert_eq!(Ok(Binary::from(vec![1, 2, 3])),
               decode(&[131, 109, 0, 0, 0, 3, 1, 2, 3]).into_binary());

    // Encode
    assert_eq!(vec![131, 109, 0, 0, 0, 3, 1, 2, 3],
               encode(Term::from(Binary::from(vec![1, 2, 3]))));
}

#[test]
fn bit_binary_test() {
    // Display
    assert_eq!("<<1,2,3>>", BitBinary::from((vec![1, 2, 3], 8)).to_string());
    assert_eq!("<<1,2>>", BitBinary::from((vec![1, 2, 3], 0)).to_string());
    assert_eq!("<<1,2,3:5>>",
               BitBinary::from((vec![1, 2, 3], 5)).to_string());

    // Decode
    assert_eq!(Ok(BitBinary::from((vec![1, 2, 3], 5))),
               decode(&[131, 77, 0, 0, 0, 3, 5, 1, 2, 24]).into_bit_binary());

    // Encode
    assert_eq!(vec![131, 77, 0, 0, 0, 3, 5, 1, 2, 24],
               encode(Term::from(BitBinary::from((vec![1, 2, 3], 5)))));
}

#[test]
fn list_test() {
    // Display
    assert_eq!("['a',1]",
               List::from(vec![Term::from(Atom::from("a")), Term::from(FixInteger::from(1))])
                   .to_string());
    assert_eq!("[]", List::nil().to_string());

    // Decode
    assert_eq!(Ok(List::nil()), decode(&[131, 106]).into_list()); // NIL_EXT
    assert_eq!(Ok(List::from(vec![Term::from(FixInteger::from(1)),
                                  Term::from(FixInteger::from(2))])),
               decode(&[131, 107, 0, 2, 1, 2]).into_list()); // STRING_EXT
    assert_eq!(Ok(List::from(vec![Term::from(Atom::from("a"))])),
               decode(&[131, 108, 0, 0, 0, 1, 100, 0, 1, 97, 106]).into_list());

    // Encode
    assert_eq!(vec![131, 106], encode(Term::from(List::nil())));
    assert_eq!(vec![131, 107, 0, 2, 1, 2],
               encode(Term::from(List::from(vec![Term::from(FixInteger::from(1)),
                                                 Term::from(FixInteger::from(2))]))));
    assert_eq!(vec![131, 108, 0, 0, 0, 1, 100, 0, 1, 97, 106],
               encode(Term::from(List::from(vec![Term::from(Atom::from("a"))]))));
}

#[test]
fn improper_list_test() {
    // Display
    assert_eq!("[0,'a'|1]",
               ImproperList::from((vec![Term::from(FixInteger::from(0)),
                                        Term::from(Atom::from("a"))],
                                   Term::from(FixInteger::from(1))))
                   .to_string());

    // Decode
    assert_eq!(Ok(ImproperList::from((vec![Term::from(Atom::from("a"))],
                                      Term::from(FixInteger::from(1))))),
               decode(&[131, 108, 0, 0, 0, 1, 100, 0, 1, 97, 97, 1]).into_improper_list());

    // Encode
    assert_eq!(vec![131, 108, 0, 0, 0, 1, 100, 0, 1, 97, 97, 1],
               encode(Term::from(ImproperList::from((vec![Term::from(Atom::from("a"))],
                                                     Term::from(FixInteger::from(1)))))));
}

#[test]
fn tuple_test() {
    // Display
    assert_eq!("{'a',1}",
               Tuple::from(vec![Term::from(Atom::from("a")), Term::from(FixInteger::from(1))])
                   .to_string());
    assert_eq!("{}", Tuple::from(vec![]).to_string());

    // Decode
    assert_eq!(Ok(Tuple::from(vec![Term::from(Atom::from("a")), Term::from(FixInteger::from(1))])),
               decode(&[131, 104, 2, 100, 0, 1, 97, 97, 1]).into_tuple());

    // Encode
    assert_eq!(vec![131, 104, 2, 100, 0, 1, 97, 97, 1],
               encode(Term::from(Tuple::from(vec![Term::from(Atom::from("a")),
                                                  Term::from(FixInteger::from(1))]))));
}

fn encode(term: Term) -> Vec<u8> {
    let mut buf = Vec::new();
    term.encode(&mut buf).unwrap();
    buf
}

fn decode(bytes: &[u8]) -> Term {
    Term::decode(Cursor::new(bytes)).unwrap()
}
