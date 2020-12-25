use serde_json::{Value, Map};

pub fn unwrap_json_object(v: Value) -> Map<String, Value> {
    if let Value::Object(obj) = v {
        return obj;
    } else {
        panic!("json root is not an object")
    };
}
