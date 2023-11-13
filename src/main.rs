use serde_bencode::de;
// use serde_bencode::ser::{to_bytes, to_string, Serializer};
use serde_bencode::value::Value;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Value {
    de::from_str(encoded_value).unwrap()

    // Convert BencodedList into serde_json::Value
}

// This is the entry point of your program

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
                    _ => (),
                };
            }
            if items.len() == 0 {
                println!("]");
            } else {
                print!("]");
            }
        } else if let Value::Int(i) = decoded_value {
            println!("{}", i);
        } else if let Value::Bytes(b) = decoded_value {
            println!("\"{}\"", String::from_utf8(b.to_vec()).unwrap());
            // print!("\n");
        } else {
            print!("unknown for this (dictionary or list)");
        }

        // println!("{:?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
