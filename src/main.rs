use serde_bencode::de;
// use serde_bencode::ser::{to_bytes, to_string, Serializer};
// use serde_bencode::ser::to_string;
use serde_bencode::value::Value;
// use serde_json::{to_string as to_string_json, to_string_pretty};
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Value {
    de::from_str(encoded_value).unwrap()

    // Convert BencodedList into serde_json::Value
}

// This is the entry point of your program
fn print_list(decoded_value: &Value, is_nested: bool) {
    if let Value::List(items) = decoded_value {
        print!("[");
        for (pos, item) in items.iter().enumerate() {
            match item {
                Value::Int(i) => {
                    print!("{}{}", i, if pos == items.len() - 1 { "" } else { "," })
                }
                Value::Bytes(b) => {
                    print!(
                        "\"{}\"{}",
                        String::from_utf8(b.to_vec()).unwrap(),
                        if pos == items.len() - 1 { "" } else { "," }
                    )
                }
                Value::List(_) => {
                    print_list(item, true);
                }
                _ => (),
            };
        }
        if items.len() == 0 {
            return print!("]");
        } else {
            print!("]");
        }
    } else if let Value::Int(i) = decoded_value {
        print!("{}", i);
    } else if let Value::Bytes(b) = decoded_value {
        return println!("\"{}\"", String::from_utf8(b.to_vec()).unwrap());
        // print!("\n");
    } else {
        print!("unknown for this (dictionary or list)");
    }
    if !is_nested {
        print!("\n")
    }
}
// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        //println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];

        let decoded_value = decode_bencoded_value(encoded_value);
        // println!("{}", to_string_pretty(&decoded_value).unwrap());
        // println!("{}", to_string(&decoded_value).unwrap());
        // println!("{}", to_string_json(&decoded_value).unwrap());
        print_list(&decoded_value, false);
        // println!("{:?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
