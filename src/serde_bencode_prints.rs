// serde_bencode parsing alternative
// #[macro_use]
// extern crate serde_derive;
// use serde;
// use serde_bencode::de;
// use serde_bencode::value::Value;
// use serde_bytes::ByteBuf;
// use std::io::{self, Read};
// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// struct Info {
//     pub name: String,
//     pub pieces: ByteBuf,
//     #[serde(rename = "piece length")]
//     pub piece_length: i64,
//     #[serde(default)]
//     pub md5sum: Option<String>,
//     #[serde(default)]
//     pub length: Option<i64>,
//     #[serde(default)]
//     pub files: Option<Vec<File>>,
//     #[serde(default)]
//     pub private: Option<u8>,
//     #[serde(default)]
//     pub path: Option<Vec<String>>,
//     #[serde(default)]
//     #[serde(rename = "root hash")]
//     pub root_hash: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// struct Torrent {
//     info: Info,
//     #[serde(default)]
//     announce: Option<String>,
//     #[serde(default)]
//     nodes: Option<Vec<Node>>,
//     #[serde(default)]
//     encoding: Option<String>,
//     #[serde(default)]
//     httpseeds: Option<Vec<String>>,
//     #[serde(default)]
//     #[serde(rename = "announce-list")]
//     announce_list: Option<Vec<Vec<String>>>,
//     #[serde(default)]
//     #[serde(rename = "creation date")]
//     creation_date: Option<i64>,
//     #[serde(rename = "comment")]
//     comment: Option<String>,
//     #[serde(default)]
//     #[serde(rename = "created by")]
//     created_by: Option<String>,
// }

// // pub fn parse_torrent_and_print(torrent_file_bytes: &[u8]) {
// //     let torrent: Torrent = serde_bencode::from_bytes(torrent_file_bytes).unwrap();
// //     println!("{:?}", torrent);
// // }

// // // pub fn print_dictionary(decoded_value: &Value, is_nested: bool) {
// // //     if let Value::Dict(items) = decoded_value {
// // //         print!("{{");
// // //         for (pos, (key, value)) in items.iter().enumerate() {
// // //             match value {
// // //                 Value::Int(i) => {
// // //                     print!("\"{}\":{}", String::from_utf8_lossy(key), i);
// // //                 }
// // //                 Value::Bytes(b) => {
// // //                     print!(
// // //                         "\"{}\":\"{}\"",
// // //                         String::from_utf8_lossy(key),
// // //                         String::from_utf8(b.to_vec()).unwrap()
// // //                     );
// // //                 }
// // //                 Value::List(_) => {
// // //                     print!("\"{}\":", String::from_utf8_lossy(key));
// // //                     print_list(value, true);
// // //                 }
// // //                 Value::Dict(_) => {
// // //                     print!("\"{}\":", String::from_utf8_lossy(key));
// // //                     print_dictionary(value, true);
// // //                 }
// // //             };
// // //             if pos != items.len() - 1 {
// // //                 print!(",");
// // //             }
// // //         }
// // //         print!("}}");
// // //     }
// // //     if !is_nested {
// // //         print!("\n")
// // //     }
// // // }

// // // // This is the entry point of your program
// // // pub fn print_list(decoded_value: &Value, is_nested: bool) {
// // //     if let Value::List(items) = decoded_value {
// // //         print!("[");
// // //         for (pos, item) in items.iter().enumerate() {
// // //             match item {
// // //                 Value::Int(i) => {
// // //                     print!("{}{}", i, if pos == items.len() - 1 { "" } else { "," })
// // //                 }
// // //                 Value::Bytes(b) => {
// // //                     print!(
// // //                         "\"{}\"{}",
// // //                         String::from_utf8(b.to_vec()).unwrap(),
// // //                         if pos == items.len() - 1 { "" } else { "," }
// // //                     )
// // //                 }
// // //                 Value::List(_) => {
// // //                     print_list(item, true);
// // //                     if pos != items.len() - 1 {
// // //                         print!(",");
// // //                     }
// // //                 }
// // //                 Value::Dict(_) => {
// // //                     print_dictionary(item, true);
// // //                     if pos != items.len() - 1 {
// // //                         print!(",");
// // //                     }
// // //                 }
// // //             };
// // //         }
// // //         print!("]")
// // //     } else {
// // //         print!("unknown type");
// // //     }
// // //     if !is_nested {
// // //         print!("\n")
// // //     }
// // // }

// // // pub fn print_decoded_value(decoded_value: &Value) {
// // //     match decoded_value {
// // //         Value::Int(i) => {
// // //             println!("{}", i)
// // //         }
// // //         Value::Bytes(b) => {
// // //             println!("\"{}\"", String::from_utf8(b.to_vec()).unwrap())
// // //         }
// // //         Value::List(_) => {
// // //             print_list(decoded_value, false);
// // //         }
// // //         Value::Dict(_) => {
// // //             print_dictionary(decoded_value, false);
// // //         }
// // //     };
// // // }
