use crate::{NbtReader, nbt_versions, NbtValue};
use super::{BorrowNbtValue, NbtBorrowTrait};


#[test]
fn hello_world_borrow() {
    let mut data: [u8; 0x21] = [
        0x0A, // TAG_Compound
        0x00, 0x0B, // root name length (11)
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, // hello world
        0x08, // TAG_String
        0x00, 0x04, // name length (4)
        0x6E, 0x61, 0x6D, 0x65, // name
        0x00, 0x09, // value length (9)
        0x42, 0x61, 0x6E, 0x61, 0x6E, 0x72, 0x61, 0x6D, 0x61, // Bananrama
        0x00, // TAG_End
    ];

    let mut reader = NbtReader::new(&mut data);

    let data = nbt_versions::Java::from_reader(&mut reader);
    if let Err(e) = data {
        println!("cursor state:\n{}", reader.show_cursor_fancy(None));
        panic!("{}", e);
    }
    
    let correct_data = NbtValue::Compound(
        Some("hello world".into()),
        vec![("name".into(), NbtValue::String("Bananrama".into()))],
    );

}
