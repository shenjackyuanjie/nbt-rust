/// 参考实现:
/// https://github.com/serde-rs/json/blob/master/src/macros.rs#L70
/// TODO
#[macro_export]
macro_rules! nbt {
    ($($nbt:tt)+) => {
        $crate::nbt_inner!($($nbt)+)
    };
}

#[macro_export]
macro_rules! nbt_inner {

    // // Done with trailing comma.
    // (@array [$($elems:expr,)*]) => {
    //     vec![$($elems,)*]
    // };
    // // Done without trailing comma.
    // (@array [$($elems:expr),*]) => {
    //     vec![$($elems),*]
    // };

    // // 解析 array

    // // next true/false
    // (@array [$($elems:expr,)*] true $($rest:tt)*) => {
    //     $crate::nbt_inner!(@array [$($elems,)* $crate::NbtValue::value_true()] $($rest)*)
    // };
    // (@array [$($elems:expr,)*] false $($rest:tt)*) => {
    //     $crate::nbt_inner!(@array [$($elems,)* $crate::NbtValue::value_false()] $($rest)*)
    // };

    // // next list
    // (@array [$($elems:expr,)*] [$($next:tt),*] $($rest:tt)*) => {
    //     $crate::nbt_inner!(@array [$($elems,)* $crate::NbtValue::List($crate::nbt_inner!(@array [] $($next)*))] $($rest)*)
    // };

    // // 下一个是跟着逗号的
    // (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
    //     $crate::nbt_inner!(@array [$($elems,)* $crate::nbt_inner!($next),] $($rest)*)
    // };
    // // 最后一个
    // (@array [$($elems:expr,)*] $last:expr) => {
    //     $crate::nbt_inner!(@array [$($elems,)* $crate::nbt_inner!($last)])
    // };
    // // 跟着逗号的 (去掉尾逗号, 开始下一个匹配)
    // (@array [$($elems:expr,)*] , $($rest:tt)*) => {
    //     $crate::nbt_inner!(@array [$($elems,)*] $($rest)*)
    // };

    // snbt 的 true & false
    (true) => {
        $crate::NbtValue::value_true()
    };
    (false) => {
        $crate::NbtValue::value_false()
    };
    // string
    // 放在 true & false 之后，因为 true & false 也是一个字面量
    ($value:literal) => {
        $crate::NbtValue::String($crate::Mutf8String::from($value))
    };
    // 空 list
    ([]) => {
        $crate::NbtValue::List(Vec::new())
    };

    // list
    ([ $($tt:tt)+ ]) => {
        $crate::NbtValue::List($crate::nbt_inner!(@array [] $($tt)+))
    };

    // 空 compound
    ({}) => {
        $crate::NbtValue::Compound(None, Vec::new())
    };
    // compound with name
    ($name:tt: { $($key:tt: $value:tt),* }) => {
        $crate::NbtValue::Compound(Some($crate::nbt_inner!($name)), vec![$((nbt!($key), $crate::nbt_inner!($value))),*])
    };
    // compound without name
    ({ $($key:tt: $value:tt),* }) => {
        $crate::NbtValue::Compound(None, vec![$(($crate::nbt_inner!($key), $crate::nbt_inner!($value))),*])
    };

}

#[cfg(test)]
mod tests {
    use crate::{nbt, NbtValue};

    #[test]
    fn just_true_false() {
        assert_eq!(nbt!(true), NbtValue::Byte(1));
        assert_eq!(nbt!(false), NbtValue::Byte(0));
    }

    #[test]
    fn just_string() {
        assert_eq!(nbt!("hello"), NbtValue::String("hello".into()));
    }

    #[test]
    fn just_list() {
        assert_eq!(nbt!([]), NbtValue::List(Vec::new()));
    }
}
