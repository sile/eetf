extern crate eetf;

use std::io;
use std::io::Cursor;
use eetf::*;

#[test]
fn atom_test() {
    // Display
    assert_eq!("'foo'", atom("foo").to_string());
    assert_eq!(r#"'fo\'o'"#, atom(r#"fo'o"#).to_string());
    assert_eq!(r#"'fo\\o'"#, atom(r#"fo\o"#).to_string());

    // Decode
    let input_list: [&[u8]; 4] = [&[131, 100, 0, 3, 102, 111, 111], // ATOM_EXT
                                  &[131, 115, 3, 102, 111, 111], // SMALL_ATOM_EXT
                                  &[131, 118, 0, 3, 102, 111, 111], // ATOM_UTF8_EXT
                                  &[131, 119, 3, 102, 111, 111]]; // SMALL_ATOM_UTF8_EXT
    for bytes in &input_list {
        let term = Term::decode(Cursor::new(bytes)).unwrap();
        assert_eq!("foo", term.as_atom().unwrap().name);
    }

    // Encode
    assert_eq!(vec![131, 100, 0, 3, 102, 111, 111],
               encode_to_bytes(&atom_term("foo")).unwrap());
}

fn encode_to_bytes(term: &Term) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    try!(term.encode(&mut buf));
    Ok(buf)
}

fn atom(name: &str) -> Atom {
    Atom { name: name.to_string() }
}

fn atom_term(name: &str) -> Term {
    Term::Atom(atom(name))
}
