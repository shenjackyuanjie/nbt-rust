

#[macro_export]
macro_rules! nbt {


    // snbt 的 true & false
    (true) => {
        $crate::Value::value_true()
    };
    (false) => {
        $crate::Value::value_false()
    }
}
