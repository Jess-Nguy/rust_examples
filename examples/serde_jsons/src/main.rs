use serde_json::{json, Map, Value};

// Testing to see how to merge 2 JSON objects.
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

    // Combine different types of values in 1 vector.
    let value = String::from("column_id");
    let string_json = json!(value);

    let mut field_labels: Vec<Value> = vec![string_json];

    let obj = json!({
        "name": "Equipment",
        "wxType": "datasource-tier-entity-columns-values",
        "nullable": false
    });

    field_labels.push(obj);

    println!("\nFIELD LABELS: {field_labels:?}");

    // json the vec of values
    let json = json!(field_labels);
    println!("JSON: {json}\n");

    // Accessing nested JSON values.
    let form: Value = json!({
        "label": "A new Score test - Score",
        "theText": {
            "name": "multipleChoiceScoreText",
            "uuid": "fe641534-8fc2-4a35-a644-34a4435d3062",
            "label": "A new Score test",
            "wxType": "wx-multiple-choice-score-label}",
            "parentId": "d7761f14-21f1-4b19-a46a-f3f95d2b9ae6"
          },
        "theScore": {
            "name": "multipleChoiceScore",
            "value": 0,
            "wxType": "wx-multiple-choice-score"
        }
    });

    let label = form["label"].as_str();
    println!("LABEL: {label:?}");

    let el_type = &form["attributes"]["wxType"]
        .as_str()
        .or_else(|| form["theText"]["wxType"].as_str());
    println!("TYPE: {el_type:?}");
}
