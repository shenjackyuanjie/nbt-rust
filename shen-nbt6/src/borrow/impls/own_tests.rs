use crate::borrow::{BorrowNbtValue as BValue, NbtBorrowTrait};
use crate::{nbt_version, NbtReader};

#[test]
fn hello_world_borrow() {
    let data: [u8; 0x21] = [
        0x0A, // TAG_Compound pos: 0
        0x00, 0x0B, // root name length (11) pos: 1
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C,
        0x64, // hello world pos: 3
        0x08, // TAG_String pos: 14
        0x00, 0x04, // name length (4) pos: 15
        0x6E, 0x61, 0x6D, 0x65, // name pos: 17
        0x00, 0x09, // value length (9) pos: 21
        0x42, 0x61, 0x6E, 0x61, 0x6E, 0x72, 0x61, 0x6D, 0x61, // Bananrama pos: 23
        0x00, // TAG_End pos: 32
    ];

    let mut reader = NbtReader::new(&data);

    let data = nbt_version::Java::from_reader(&mut reader).unwrap();
    println!("{:#?}", data);

    let owned_data = nbt_version::Java::read_data(&data, &mut reader);
    println!("{}", owned_data);

    panic!()
}
