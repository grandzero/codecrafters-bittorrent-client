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
            items.iter().for_each(|item| match item {
                Value::Int(i) => print!("{}, ", i),
                Value::Bytes(b) => print!("{:?}, ", String::from_utf8(b.to_vec()).unwrap()),
                _ => (),
            });
            print!("]");
        } else if let Value::Int(i) = decoded_value {
            print!("{}", i);
        } else if let Value::Bytes(b) = decoded_value {
            print!("{:?}", String::from_utf8(b.to_vec()).unwrap());
        } else {
            print!("unknown");
        }

        // println!("{:?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
