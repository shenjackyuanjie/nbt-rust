use crate::borrow::NbtBorrowTrait;
use crate::tests::{BIG_TEST_DATA, COMPLEX_PLR_DATA, HELLO_WORLD_DATA};
use crate::{nbt_version, NbtReader, NbtValue};

#[test]
fn hello_world() {
    let mut reader = NbtReader::new(&HELLO_WORLD_DATA);

    let data = nbt_version::Java::from_reader(&mut reader).unwrap();
    let owned_data = nbt_version::Java::read_data(&data, &mut reader);
    println!("{}", owned_data);

    let expect_data = NbtValue::Compound(
        Some("hello world".into()),
        vec![("name".into(), NbtValue::String("Bananrama".into()))],
    );

    assert_eq!(owned_data, expect_data);
}

#[test]
fn big_test() {
    let mut reader = NbtReader::new(&BIG_TEST_DATA);

    let data = nbt_version::Java::from_reader(&mut reader).unwrap();
    let owned_data = nbt_version::Java::read_data(&data, &mut reader);
    println!("{}", owned_data.display_data());

    let expect_data = NbtValue::Compound(
        Some("Level".into()),
        vec![
            ("longTest".into(), NbtValue::Long(9223372036854775807)),
            ("shortTest".into(), NbtValue::Short(32767)),
            (
                "stringTest".into(),
                NbtValue::String("HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".into()),
            ),
            ("floatTest".into(), NbtValue::Float(0.49823147)),
            ("intTest".into(), NbtValue::Int(2147483647)),
            (
                "nested compound test".into(),
                NbtValue::Compound(
                    None,
                    vec![
                        (
                            "ham".into(),
                            NbtValue::Compound(
                                None,
                                vec![
                                    ("name".into(), NbtValue::String("Hampus".into())),
                                    ("value".into(), NbtValue::Float(0.75)),
                                ],
                            ),
                        ),
                        (
                            "egg".into(),
                            NbtValue::Compound(
                                None,
                                vec![
                                    ("name".into(), NbtValue::String("Eggbert".into())),
                                    ("value".into(), NbtValue::Float(0.5)),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
            (
                "listTest (long)".into(),
                NbtValue::List(vec![
                    NbtValue::Long(11),
                    NbtValue::Long(12),
                    NbtValue::Long(13),
                    NbtValue::Long(14),
                    NbtValue::Long(15),
                ]),
            ),
            (
                "listTest (compound)".into(),
                NbtValue::List(vec![
                    NbtValue::Compound(
                        None,
                        vec![
                            ("name".into(), NbtValue::String("Compound tag #0".into())),
                            ("created-on".into(), NbtValue::Long(1264099775885)),
                        ],
                    ),
                    NbtValue::Compound(
                        None,
                        vec![
                            ("name".into(), NbtValue::String("Compound tag #1".into())),
                            ("created-on".into(), NbtValue::Long(1264099775885)),
                        ],
                    )
                ])
            ),
            (
                "byteTest".into(),
                NbtValue::Byte(127),
            )           ,
            (
                "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))".into(),
                NbtValue::ByteArray(vec![ 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48, 0, 62, 34, 16, 8, 10, 22, 44, 76, 18, 70, 32, 4, 86, 78, 80, 92, 14, 46, 88, 40, 2, 74, 56, 48, 50, 62, 84, 16, 58, 10, 72, 44, 26, 18, 20, 32, 54, 86, 28, 80, 42, 14, 96, 88, 90, 2, 24, 56, 98, 50, 12, 84, 66, 58, 60, 72, 94, 26, 68, 20, 82, 54, 36, 28, 30, 42, 64, 96, 38, 90, 52, 24, 6, 98, 0, 12, 34, 66, 8, 60, 22, 94, 76, 68, 70, 82, 4, 36, 78, 30, 92, 64, 46, 38, 40, 52, 74, 6, 48
                ])
            ),
            (
                "doubleTest".into(),
                NbtValue::Double(0.4931287132182315),
            )
        ],
    );

    assert_eq!(owned_data, expect_data);
}

#[test]
fn complex_player() {
    let mut reader = NbtReader::new(&COMPLEX_PLR_DATA);

    let data = nbt_version::Java::from_reader(&mut reader).unwrap();
    let owned_data = nbt_version::Java::read_data(&data, &mut reader);
    println!("{}", owned_data.display_data());

    panic!();
}