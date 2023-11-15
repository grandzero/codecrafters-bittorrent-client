use bendy::decoding::{Error, FromBencode, Object};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BencodeValue {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(Vec<(String, BencodeValue)>),
}

pub enum ParseError {
    InvalidFormat,
}

impl FromBencode for BencodeValue {
    // const EXPECTED_RECURSION_DEPTH: usize = 1;

    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut dict_vec: Vec<(String, BencodeValue)> = Vec::new();
        let mut list_vec: Vec<BencodeValue> = Vec::new();
        match object {
            Object::Integer(_i) => {
                //return Ok(BencodeValue::Int(_i.parse::<i64>().unwrap()));
                return object
                    .try_into_integer()
                    .and_then(|x| Ok(BencodeValue::Int(x.parse::<i64>().unwrap())));
            }
            Object::Bytes(_b) => {
                //return Ok(BencodeValue::Bytes(_b.to_vec()));
                return object
                    .try_into_bytes()
                    .and_then(|x| Ok(BencodeValue::Bytes(x.to_vec())));
            }
            Object::List(mut ls) => {
                while let Ok(item) = ls.next_object() {
                    if let Some(value) = item {
                        let i = Self::decode_bencode_object(value)?;
                        list_vec.push(i);
                    } else {
                        break;
                    }
                }
                return Ok(BencodeValue::List(list_vec));
            }
            Object::Dict(mut dict) => {
                while let Some(pair) = dict.next_pair().ok() {
                    if let Some((key, value)) = pair {
                        dict_vec.push((
                            String::from_utf8(key.to_vec()).unwrap(),
                            Self::decode_bencode_object(value)?,
                        ));
                    } else {
                        break;
                    }
                }
                return Ok(BencodeValue::Dict(dict_vec));
            }
        }
    }
}
fn print_dictionary(decoded_value: &BencodeValue, is_nested: bool) {
    if let BencodeValue::Dict(items) = decoded_value {
        print!("{{");
        for (pos, (key, value)) in items.iter().enumerate() {
            match value {
                BencodeValue::Int(i) => {
                    print!("\"{}\":{}", key, i);
                }
                BencodeValue::Bytes(b) => {
                    print!("\"{}\":\"{}\"", key, String::from_utf8(b.to_vec()).unwrap());
                }
                BencodeValue::List(_) => {
                    print!("\"{}\":", key);
                    print_list(value, true);
                }
                BencodeValue::Dict(_) => {
                    print!("\"{}\":", key);
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
fn print_list(decoded_value: &BencodeValue, is_nested: bool) {
    if let BencodeValue::List(items) = decoded_value {
        print!("[");
        for (pos, item) in items.iter().enumerate() {
            match item {
                BencodeValue::Int(i) => {
                    print!("{}{}", i, if pos == items.len() - 1 { "" } else { "," })
                }
                BencodeValue::Bytes(b) => {
                    print!(
                        "\"{}\"{}",
                        String::from_utf8(b.to_vec()).unwrap(),
                        if pos == items.len() - 1 { "" } else { "," }
                    )
                }
                BencodeValue::List(_) => {
                    print_list(item, true);
                    if pos != items.len() - 1 {
                        print!(",");
                    }
                }
                BencodeValue::Dict(_) => {
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

pub fn print_decoded_value(decoded_value: &BencodeValue) {
    match decoded_value {
        BencodeValue::Int(i) => {
            println!("{}", i)
        }
        BencodeValue::Bytes(b) => {
            println!("\"{}\"", String::from_utf8(b.to_vec()).unwrap())
        }
        BencodeValue::List(_) => {
            print_list(decoded_value, false);
        }
        BencodeValue::Dict(_) => {
            print_dictionary(decoded_value, false);
        }
    };
}

pub fn decode_bn(encoded_value: &str) -> Result<BencodeValue, ParseError> {
    if let Ok(val) = BencodeValue::from_bencode(encoded_value.as_bytes()) {
        print_decoded_value(&val);
        return Ok(val);
    } else {
        return Err(ParseError::InvalidFormat);
    }
}

// pub enum BencodeValue {
//     Int(i64),
//     Bytes(Vec<u8>),
//     List(Vec<BencodeValue>),
//     Dict(Vec<(Vec<u8>, BencodeValue)>),
// }
// #[derive(Debug)]
// pub enum ParseError {
//     InvalidType,
//     InvalidFormat,
//     InvalidValue,
// }

// pub fn recursive_bencoded_decode_value(encoded_value: &str) -> Result<BencodeValue, ParseError> {
//     match &encoded_value.chars().next().unwrap() {
//         'i' => {
//             if let Some(result_val) = encoded_value[1..]
//                 .split_once('e')
//                 .and_then(|x| {
//                     println!("x => {:?}", x.0.parse::<i64>().ok());
//                     x.0.parse::<i64>().ok()
//                 })
//                 .and_then(|res| Some(BencodeValue::Int(res)))
//             {
//                 println!("Result {:?}", result_val);
//                 return Ok(result_val);
//             } else {
//                 return Err(ParseError::InvalidFormat);
//             }
//         }
//         // 'l' => Ok(BencodeValue::List(list)),
//         // // 'd' => {
//         // //     let mut value = encoded_value[1..].split('e');
//         // //     let value = value.next().unwrap();
//         // //     let mut dict = Vec::new();
//         // //     for item in value.split(',') {
//         // //         let mut item = item.split(':');
//         // //         let key = item.next().unwrap();
//         // //         let value = item.next().unwrap();
//         // //         dict.push((
//         // //             key.as_bytes().to_vec(),
//         // //             custom_bencoded_decode_value(value)?,
//         // //         ));
//         // //     }
//         // //     Ok(BencodeValue::Dict(dict))
//         // // }
//         '0'..='9' => {
//             if let Some(str_value) = &encoded_value[..].split_once(':').and_then(|x| {
//                 if let Some(len) = x.0.parse::<usize>().ok() {
//                     if x.1.len() <= len {
//                         return Some(&x.1[..len]);
//                     }
//                 }
//                 None
//             }) {
//                 return Ok(BencodeValue::Bytes(str_value.as_bytes().to_vec()));
//             } else {
//                 return Err(ParseError::InvalidFormat);
//             }
//         }
//         _ => {
//             return Err(ParseError::InvalidValue);
//         }
//     }
// }

// fn print_dictionary(decoded_value: &BencodeValue, is_nested: bool) {
//     if let BencodeValue::Dict(items) = decoded_value {
//         print!("{{");
//         for (pos, (key, value)) in items.iter().enumerate() {
//             match value {
//                 BencodeValue::Int(i) => {
//                     print!("\"{}\":{}", String::from_utf8(key.to_vec()).unwrap(), i);
//                 }
//                 BencodeValue::Bytes(b) => {
//                     print!(
//                         "\"{}\":\"{}\"",
//                         String::from_utf8(key.to_vec()).unwrap(),
//                         String::from_utf8(b.to_vec()).unwrap()
//                     );
//                 }
//                 BencodeValue::List(_) => {
//                     print!("\"{}\":", String::from_utf8(key.to_vec()).unwrap(),);
//                     print_list(value, true);
//                 }
//                 BencodeValue::Dict(_) => {
//                     print!("\"{}\":", String::from_utf8(key.to_vec()).unwrap(),);
//                     print_dictionary(value, true);
//                 }
//             };
//             if pos != items.len() - 1 {
//                 print!(",");
//             }
//         }
//         print!("}}");
//     }
//     if !is_nested {
//         print!("\n")
//     }
// }

// // This is the entry point of your program
// fn print_list(decoded_value: &BencodeValue, is_nested: bool) {
//     if let BencodeValue::List(items) = decoded_value {
//         print!("[");
//         for (pos, item) in items.iter().enumerate() {
//             match item {
//                 BencodeValue::Int(i) => {
//                     print!("{}{}", i, if pos == items.len() - 1 { "" } else { "," })
//                 }
//                 BencodeValue::Bytes(b) => {
//                     print!(
//                         "\"{}\"{}",
//                         String::from_utf8(b.to_vec()).unwrap(),
//                         if pos == items.len() - 1 { "" } else { "," }
//                     )
//                 }
//                 BencodeValue::List(_) => {
//                     print_list(item, true);
//                     if pos != items.len() - 1 {
//                         print!(",");
//                     }
//                 }
//                 BencodeValue::Dict(_) => {
//                     print_dictionary(item, true);
//                     if pos != items.len() - 1 {
//                         print!(",");
//                     }
//                 }
//             };
//         }
//         print!("]")
//         // if items.len() == 0 {
//         //     return println!("]");
//         // } else {
//         //     print!("]");
//         // }
//     } else {
//         print!("unknown type");
//     }
//     if !is_nested {
//         print!("\n")
//     }
// }

// pub fn print_decoded_value(decoded_value: &BencodeValue) {
//     match decoded_value {
//         BencodeValue::Int(i) => {
//             println!("{}", i)
//         }
//         BencodeValue::Bytes(b) => {
//             println!("\"{}\"", String::from_utf8(b.to_vec()).unwrap())
//         }
//         BencodeValue::List(_) => {
//             print_list(decoded_value, false);
//         }
//         BencodeValue::Dict(_) => {
//             print_dictionary(decoded_value, false);
//         }
//     };
// }
