use serde_bencode;
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

fn print_dictionary(decoded_value: &Value, is_nested: bool) {
    if let Value::Dict(items) = decoded_value {
        print!("{{");
        for (pos, (key, value)) in items.iter().enumerate() {
            match value {
                Value::Int(i) => {
                    print!("\"{}\":{}", String::from_utf8(key.to_vec()).unwrap(), i);
                }
                Value::Bytes(b) => {
                    print!(
                        "\"{}\":\"{}\"",
                        String::from_utf8(key.to_vec()).unwrap(),
                        String::from_utf8(b.to_vec()).unwrap()
                    );
                }
                Value::List(_) => {
                    print!("\"{}\":", String::from_utf8(key.to_vec()).unwrap(),);
                    print_list(value, true);
                }
                Value::Dict(_) => {
                    print!("\"{}\":", String::from_utf8(key.to_vec()).unwrap(),);
                    print_dictionary(value, true);
                }
            };
            if pos != items.len() - 1 {
                print!(",");
            }
        }
        print!("}}");
    }
    if !is_nested {
        print!("\n")
    }
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
                    if pos != items.len() - 1 {
                        print!(",");
                    }
                }
                Value::Dict(_) => {
                    print_dictionary(item, true);
                    if pos != items.len() - 1 {
                        print!(",");
                    }
                }
            };
        }
        print!("]")
        // if items.len() == 0 {
        //     return println!("]");
        // } else {
        //     print!("]");
        // }
    } else {
        print!("unknown type");
    }
    if !is_nested {
        print!("\n")
    }
}

fn print_decoded_value(decoded_value: &Value) {
    match decoded_value {
        Value::Int(i) => {
            println!("{}", i)
        }
        Value::Bytes(b) => {
            println!("{}", String::from_utf8(b.to_vec()).unwrap())
        }
        Value::List(_) => {
            print_list(decoded_value, false);
        }
        Value::Dict(_) => {
            print_dictionary(decoded_value, false);
        }
    };
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

        print_decoded_value(&decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
