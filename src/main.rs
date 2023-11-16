mod custom_bencode_decode;
mod serde_bencode_prints;
use custom_bencode_decode::decode_bn;
use custom_bencode_decode::decode_torrent;
// use serde_bencode;
// use serde_bencode::de;
// use serde_bencode::value::Value;
use std::fs;

use std::env;
// Serde bencode parsing in alternative
// #[allow(dead_code)]
// fn decode_bencoded_value(encoded_value: &[u8]) -> Value {
//     de::from_bytes(encoded_value).unwrap()
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let _ = decode_bn(encoded_value);
    } else if command == "info" {
        let torrent_content_as_bytes = fs::read(&args[2]).unwrap();
        // println!("{:?}", String::from_utf8_lossy(&torrent_content_as_bytes));
        //let decoded_value = decode_bencoded_value(&torrent_content_as_bytes);
        //print_decoded_value(&decoded_value);
        if let Ok(custom_torrent) = decode_torrent(&torrent_content_as_bytes) {
            println!("{}", custom_torrent);
        } else {
            println!("Error");
        }
        //println!("{:?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
