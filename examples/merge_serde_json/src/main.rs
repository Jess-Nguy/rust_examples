use serde_json::{json, Map, Value};

fn merge_json_values(
    mut base: Map<String, Value>,
    additional: Map<String, Value>,
) -> Map<String, Value> {
    for (key, value) in additional {
        base.insert(key, value);
    }
    base
}

fn main() {
    let json1 = json!({
        "name": "Alice",
        "age": 30,
        "city": "New York"
    });

    let json2 = json!({
        "age": 31,
        "job": "Engineer"
    });

    if let (Value::Object(obj1), Value::Object(obj2)) = (json1, json2) {
        let merged = merge_json_values(obj1, obj2);
        let merged_json = serde_json::to_string_pretty(&Value::Object(merged)).unwrap();
        println!("{}", merged_json);
    }
}
