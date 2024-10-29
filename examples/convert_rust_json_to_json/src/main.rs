use colored::Colorize;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
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

    println!("\nWith text:\n{}\n", contents.yellow());
    contents
}

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
