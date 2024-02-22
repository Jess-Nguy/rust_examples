use serde_json::{json, Map, Number as JsonNumber, Value as JsonValue};
use serde_yaml::Value as YamlValue;

fn convert_tag_to_string(value: &YamlValue) -> JsonValue {
    fn convert_key(key: &str) -> String {
        if key.starts_with(":") {
            key[1..].to_string()
        } else {
            key.to_string()
        }
    }

    match value {
        YamlValue::Mapping(map) => {
            println!("\n Mapping: {:?}", map);
            let mut new_map = Map::new();
            for (k, v) in map.iter() {
                if let YamlValue::String(key) = k {
                    let new_key = convert_key(key);
                    new_map.insert(new_key, convert_tag_to_string(v));
                }
            }
            JsonValue::Object(new_map)
        }
        YamlValue::Sequence(seq) => {
            let mut new_seq = Vec::new();
            for v in seq.iter() {
                new_seq.push(convert_tag_to_string(v));
            }
            JsonValue::Array(new_seq)
        }
        YamlValue::String(s) => JsonValue::String(s.clone()),
        YamlValue::Number(n) => {
            if let Some(n) = n.as_i64() {
                JsonValue::Number(JsonNumber::from(n))
            } else if let Some(n) = n.as_f64() {
                JsonValue::Number(JsonNumber::from_f64(n).unwrap_or_else(|| JsonNumber::from(0)))
            } else {
                JsonValue::Null
            }
        }
        YamlValue::Bool(b) => JsonValue::Bool(*b),
        YamlValue::Null => JsonValue::Null,
        YamlValue::Tagged(tagged_value) => {
            json!({
                "tag": tagged_value.tag.to_string(),
                "value": convert_tag_to_string(&tagged_value.value)
            })
        }
    }
}

pub fn run(yaml_data: &str) -> JsonValue {
    let deserialized_data: YamlValue =
        serde_yaml::from_str(yaml_data).expect("Failed to parse YAML");

    let converted_data = convert_tag_to_string(&deserialized_data);

    let json_data =
        serde_json::to_string_pretty(&converted_data).expect("Failed to convert to JSON");

    println!("\nJSON data in run:: {}", json_data);
    converted_data
}

#[cfg(test)]
mod test_yaml_to_json {
    use super::*;

    #[test]
    fn test_yaml_to_json_photo_upload() {
        let yaml_data = r#"---
            :form_version: 1
            :process_id: bkcda100-9020-4c52-9ec4-11168df157bd
            :values:
                eventdescription: ''
                photoupload: !ruby/hash:ActiveSupport::HashWithIndifferentAccess
                    is_locked: false
                    signed_by: ''
                    signed_by_with_email: ''
                    timestamp: ''
                    meaning_of_signature: ''
                    value: []
                category: ''
                selectallthatapply: ''
                infobox: ''
                infobox_1: ''
                disposition: ''
                priority: ''
                infobox_2: ''
                shortterm: ''
                longterm: ''
                status: ''
                workperformedby: ''
                workperformedon: ''
                timetoperformworkexample0030: ''
            :user_id: 0
            :created_at: "2021-10-10 14:19:16 UTC"
            :updated_at: "2022-11-19 15:34:16 UTC"
        "#;

        let json_expect = json!(
            {
                "form_version": 1,
                "process_id": "bkcda100-9020-4c52-9ec4-11168df157bd",
                "user_id": 0,
                "values": {
                    "eventdescription": "",
                    "photoupload": {
                        "tag": "!ruby/hash:ActiveSupport::HashWithIndifferentAccess",
                        "value": {
                          "is_locked": false,
                          "meaning_of_signature": "",
                          "signed_by": "",
                          "signed_by_with_email": "",
                          "timestamp": "",
                          "value": []
                        }
                    },
                    "category": "",
                    "selectallthatapply": "",
                    "infobox": "",
                    "infobox_1": "",
                    "disposition": "",
                    "priority": "",
                    "infobox_2": "",
                    "shortterm": "",
                    "longterm": "",
                    "status": "",
                    "workperformedby": "",
                    "workperformedon": "",
                    "timetoperformworkexample0030": "",
                },
                "created_at": "2021-10-10 14:19:16 UTC",
                "updated_at": "2022-11-19 15:34:16 UTC"
              }
        );

        let json_data = run(yaml_data);

        assert_eq!(json_data, json_expect);
    }
}
