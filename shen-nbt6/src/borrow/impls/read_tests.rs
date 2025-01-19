use crate::borrow::{BorrowNbtValue as BValue, NbtBorrowTrait};
use crate::tests::{BIG_TEST_DATA, HELLO_WORLD_DATA};
use crate::{nbt_version, NbtReader};

#[test]
fn hello_world_borrow() {
    let mut reader = NbtReader::new(&HELLO_WORLD_DATA);

    let data = nbt_version::Java::from_reader(&mut reader);
    if let Err(e) = data {
        println!("cursor state:\n{}", reader.show_cursor_fancy(None));
        panic!("{}", e);
    }
    let correct_data = BValue::Compound(0, Some(11), vec![(17, 4, BValue::String(23, 9))]);
    assert_eq!(data.unwrap(), correct_data);
}

#[test]
fn big_test() {
    let mut reader = NbtReader::new(&BIG_TEST_DATA);

    let data = nbt_version::Java::from_reader(&mut reader);
    if let Err(e) = data {
        println!("cursor state:\n{}", reader.show_cursor_fancy(None));
        panic!("{}", e);
    }
    let correct_data = BValue::Compound(
        0,
        Some(5),
        vec![
            (11, 8, BValue::Long(19)),
            (30, 9, BValue::Short(39)),
            (44, 10, BValue::String(56, 41)),
            (100, 9, BValue::Float(109)),
            (116, 7, BValue::Int(123)),
            BValue::sub_compound(
                130,
                20,
                150,
                vec![
                    BValue::sub_compound(
                        153,
                        3,
                        156,
                        vec![(159, 4, BValue::String(165, 6)), (174, 5, BValue::Float(179))],
                    ),
                    BValue::sub_compound(
                        187,
                        3,
                        190,
                        vec![(193, 4, BValue::String(199, 7)), (209, 5, BValue::Float(214))],
                    ),
                ],
            ),
            BValue::sub_list(
                223,
                15,
                239,
                5,
                4,
                vec![
                    BValue::Long(243),
                    BValue::Long(251),
                    BValue::Long(259),
                    BValue::Long(267),
                    BValue::Long(275),
                ],
            ),
            (
                286,
                19,
                BValue::List(
                    306,
                    2,
                    10,
                    vec![
                        BValue::nameless_compound(
                            310,
                            vec![(313, 4, BValue::String(319, 15)), (337, 10, BValue::Long(347))],
                        ),
                        BValue::nameless_compound(
                            356,
                            vec![(359, 4, BValue::String(365, 15)), (383, 10, BValue::Long(393))],
                        ),
                    ],
                ),
            ),
            (405, 8, BValue::Byte(413)),
            (417, 101, BValue::ByteArray(522, 1000)),
            (1525, 10, BValue::Double(1535)),
        ],
    );
    let data = data.unwrap();
    println!("data: {:#?}", data);
    assert_eq!(data, correct_data);
}
