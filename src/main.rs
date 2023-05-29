use core::borrow;
use std::fs::File;
use std::io::{self, Write};

use serde_json::Value;

fn main() {
    fn json_builder(field: String, value: String) -> String {
        let json_string = format!(
            r#"
            {{
                "{}": "{}"
            }}
        "#,
            field, value
        );
        let json_immutable = json_string.as_str();

        let result: Result<Value, serde_json::Error> = serde_json::from_str(json_immutable);
        match result {
            Ok(value) => {
                // Use the value here
                println!("Parsed JSON: {:?}", value);
                value.to_string()
            }
            Err(error) => {
                // Handle the error here
                println!("input was {}", json_immutable);
                println!("Error parsing JSON: {}", error);
                String::from("")
            }
        }
    }
    let mut input = String::new();

    println!("Enter your name: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You typed: {}", input);
    let trimmed = String::from(input.trim());

    let file_name = format!("{}.json", input);

    print!("borrowed_input: {}", trimmed);
    let contents = json_builder(String::from("firstName"), trimmed);

    let file_name = format!("{}.json", input);
    let mut file = File::create(file_name).expect("file not found");

    file.write(contents.as_bytes())
        .expect("something went wrong reading the file");

    println!("Hello, world!");
}
