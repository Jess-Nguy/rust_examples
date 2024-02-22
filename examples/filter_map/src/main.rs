// use chrono::{DateTime, Utc};
// use serde_json::Value;
// use uuid::Uuid;

// Date_and_time
// UniqueID
fn main() {
    // let label = String::from("label 1 ?another One 2 mayBe _ #?@");
    let label = String::from("Date_and_time");

    let camel = text_to_camel(label.clone());
    println!("LABEL: {:?} -> CAMEL {:?} ", label, camel);
}

fn text_to_camel(label: String) -> String {
    let binding = label.clone().trim().to_lowercase();
    let mut chars = binding.chars().peekable();
    let mut camel = String::new();

    let mut capitalize = true;
    loop {
        println!("peek : {:?}", chars.peek());
        match chars.next() {
            Some(c) => {
                if c == ' ' || c == '-' || c == '_' {
                    if let Some(future_c) = chars.peek() {
                        if future_c.is_alphanumeric() {
                            capitalize = true;
                        }
                    }
                } else if capitalize {
                    camel.push(c.to_ascii_uppercase());
                    capitalize = false;
                } else {
                    camel.push(c);
                }
            }
            None => break,
        }
    }
    return camel;
}
