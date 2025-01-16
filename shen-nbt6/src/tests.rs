use crate::reader::NbtReader;

#[test]
fn fancy_cursor_display() {
    let datas = (1..100_u8).collect::<Vec<u8>>();
    let mut reader = NbtReader::new(&datas);
    let fancy = reader.show_cursor_fancy(None);
    let expect = "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, ....]\n ^^^^ pos: 0";
    assert_eq!(fancy, expect);

    let _ = reader.roll_down(5);
    let fancy = reader.show_cursor_fancy(None);
    let expect = "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, ....]\n                     ^^^^ pos: 5";
    assert_eq!(fancy, expect);
}
