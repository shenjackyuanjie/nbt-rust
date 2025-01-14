use crate::NbtTypeId;

/// 把 u8 转换成对应的 Nbt 类型名称
pub trait NbtTypeConversion {
    /// 把 u8 转换成对应的 Nbt 类型名称
    fn as_nbt_type_name(&self) -> String;
}

impl NbtTypeConversion for NbtTypeId {
    fn as_nbt_type_name(&self) -> String {
        if *self > 12 {
            return format!("未知类型({})", *self);
        }
        match *self {
            0 => "NBT_End(0)",
            1 => "NBT_Byte(1)",
            2 => "NBT_Short(2)",
            3 => "NBT_Int(3)",
            4 => "NBT_Long(4)",
            5 => "NBT_Float(5)",
            6 => "NBT_Double(6)",
            7 => "NBT_ByteArray(7)",
            8 => "NBT_String(8)",
            9 => "NBT_List(9)",
            10 => "NBT_Compound(10)",
            11 => "NBT_IntArray(11)",
            12 => "NBT_LongArray(12)",
            _ => unreachable!(),
        }
        .to_string()
    }
}
