extern crate eetf;

use std::io::Cursor;
use eetf::*;

#[test]
fn atom_test() {
    // Display
    assert_eq!("'foo'", Atom { name: "foo".to_string() }.to_string());
    assert_eq!(r#"'fo\'o'"#,
               Atom { name: r#"fo'o"#.to_string() }.to_string());
    assert_eq!(r#"'fo\\o'"#,
               Atom { name: r#"fo\o"#.to_string() }.to_string());

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
}
