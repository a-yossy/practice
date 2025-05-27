use std::collections::HashMap;

mod macros;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Boolean(value)
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(value)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(value: &'a str) -> Self {
        Json::String(value.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ($($t:ident)*) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Self {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128
                        usize isize f32 f64);

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = json!(
        [{
            "pitch": 440.0
        }]
    );
    let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
        vec![("pitch".to_string(), Json::Number(440.0))]
            .into_iter()
            .collect(),
    ))]);

    assert_eq!(macro_generated_value, hand_coded_value);
}

// macro_rules! setup_req {
//     ($req:ident, $server_socket:ident) => {
//         let $req = ServerRequest::new($server_socket.session());
//     };
// }

// fn handle_http_request(server_socket: &ServerSocket) {
//     setup_req!(req, server_socket);
// }

fn main() {
    let width = 4.0;
    let _desc = json!({
        "width": width,
        "height": (width * 9.0 / 4.0)
    });

    let fields = "Fields, W.C.";
    let _role = json!({
        "name": "Larson E. Whipsnade",
        "actor": fields
    });
}
