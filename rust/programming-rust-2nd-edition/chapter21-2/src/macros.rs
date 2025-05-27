pub use std::boxed::Box;
pub use std::collections::HashMap;
pub use std::string::ToString;

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([$($element:tt),*]) => {
        $crate::Json::Array(vec![$(json!($element)),*])
    };
    // ({$($key:tt : $value:tt), *}) => {
    //     Json::Object(Box::new(vec![
    //         $(($key.to_string(), json!($value))),*
    //     ].into_iter().collect()))
    // };
    ({$($key:tt : $value:tt),*}) => {
        {
            let mut fields = $crate::macros::Box::new($crate::macros::HashMap::new());
            $(fields.insert($crate::macros::ToString::to_string($key), json!($value));)*
            $crate::Json::Object(fields)
        }
    };
    ($other:tt) => {
        $crate::Json::from($other)
    };
}
