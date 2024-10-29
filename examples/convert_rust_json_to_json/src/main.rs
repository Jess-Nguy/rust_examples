use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseItem {
    name: String,
    age: i32,
    city: String,
}
fn main() {
    // See alternative JSON conversions
    // serde_to_string();
    let contents: String = read_file();
    let basic_json = convert_rust_serde_to_general_json(&contents);
    write_file(basic_json);
}

/// Read in file
fn read_file() -> String {
    // run with vscode run button
    // let file_path = "./examples/convert_rust_json_to_json/src/rust_json.txt";
    let file_path = "./src/rust_json.txt";

    println!("Read in file {}", file_path.blue());

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to READ the EXISTING file");

    // Alternative JSON conversions for serde_json value to json format. it doesn't remove the Object and Array keywords and all other rust specific keywords.
    // let new_string_pretty: String = serde_json::to_string_pretty(&contents).unwrap();
    // println!(
    //     "Serde_json::Value to JSON format? : {}",
    //     format!("{}", new_string_pretty).yellow()
    // );

    println!("\nWith text:\n{}\n", contents.yellow());

    contents
}

/// Case: printing out Value serde json to general json
/// Convert Rust serde json to general json
fn convert_rust_serde_to_general_json(input: &str) -> String {
    let mut output = input.to_string();

    // Replace Bool(true) and Bool(false)
    output = Regex::new(r"Bool\((true|false)\)")
        .unwrap()
        .replace_all(&output, "$1")
        .into();

    // Replace String("...") with "..."
    output = Regex::new(r#"String\s*\(\s*"(.*?)"\s*\)"#)
        .unwrap()
        .replace_all(&output, "\"$1\"")
        .into();

    // Replace Number(...) with the number itself
    output = Regex::new(r"Number\((\d+)\)")
        .unwrap()
        .replace_all(&output, "$1")
        .into();

    // Remove Array and Object keywords
    output = Regex::new(r"\b(Array|Object)\s*")
        .unwrap()
        .replace_all(&output, "")
        .into();

    // Replace Null with null
    output = output.replace("Null", "null");

    output
}

/// Write basic serde value json to file
fn write_file(basic_json: String) {
    // run with vscode run button
    // let file_path = "./examples/convert_rust_json_to_json/src/basic_json.json";
    let file_path = "./src/basic_json.json";
    // --snip--
    println!("Write in file {}", file_path.blue());

    let mut new_file =
        File::create_new(file_path).expect("Should have been able to CREATE the NEW file");

    new_file
        .write_all(basic_json.as_bytes())
        .expect("Should have been able to WRITE to the file");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to READ the NEW file");

    println!("\nNew file has text:\n{}\n", contents.yellow());
}

// Alternative JSON conversions but only works for rust struct to json format. it doesn't remove the Object and Array keywords and all other rust specific keywords.
fn serde_to_string() {
    let item = DatabaseItem {
        name: "Alice".to_string(),
        age: 30,
        city: "New York".to_string(),
    };
    println!("Item: {}", format!("{:?}", item).blue());

    let slice_string_in_json_format: String = serde_json::to_string(&item).unwrap();

    println!(
        "Slice String in JSON format: {}",
        format!("{}", slice_string_in_json_format).red()
    );

    let new_string_pretty: String = serde_json::to_string_pretty(&item).unwrap();

    println!(
        "New String in JSON format: {}",
        format!("{}", new_string_pretty).yellow()
    );
}
