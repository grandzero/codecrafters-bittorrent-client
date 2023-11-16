use anyhow::Result;
use bendy::{
    decoding::{Error, FromBencode, Object, ResultExt},
    encoding::AsString,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BencodeValue {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(Vec<(String, BencodeValue)>),
}
#[derive(Debug)]
pub struct MetaInfo {
    pub announce: String,
    pub info: Info,
    pub created_by: Option<String>,
    // pub comment: Option<String>,         // not official element
    // pub creation_date: Option<u64>,      // not official element
    // pub http_seeds: Option<Vec<String>>, // not official element
}
#[derive(Debug)]
pub struct Info {
    pub name: String,
    pub piece_length: String,
    pub pieces: Vec<u8>,
    pub file_length: String,
}

pub enum ParseError {
    InvalidFormat,
}

impl FromBencode for MetaInfo {
    // Try to parse with a `max_depth` of two.
    //
    // The required max depth of a data structure is calculated as follows:
    //
    //  - Every potential nesting level encoded as bencode dictionary  or list count as +1,
    //  - everything else is ignored.
    //
    // This typically means that we only need to count the amount of nested structs and container
    // types. (Potentially ignoring lists of bytes as they are normally encoded as strings.)
    //
    // struct MetaInfo {                    // encoded as dictionary (+1)
    //    announce: String,
    //    info: Info {                      // encoded as dictionary (+1)
    //      piece_length: String,
    //      pieces: Vec<u8>,                // encoded as string and therefore ignored
    //      name: String,
    //      file_length: String,
    //    },
    //    comment: Option<String>,
    //    creation_date: Option<u64>,
    //    http_seeds: Option<Vec<String>>   // if available encoded as list but even then doesn't
    //                                         increase the limit over the deepest chain including
    //                                         info
    // }
    const EXPECTED_RECURSION_DEPTH: usize = Info::EXPECTED_RECURSION_DEPTH + 1;

    /// Entry point for decoding a torrent. The dictionary is parsed for all
    /// non-optional and optional fields. Missing optional fields are ignored
    /// but any other missing fields result in stopping the decoding and in
    /// spawning [`DecodingError::MissingField`].
    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut announce = None;
        let mut created_by = None;
        // let mut creation_date = None;
        // let mut http_seeds = None;
        let mut info = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"announce", value) => {
                    announce = String::decode_bencode_object(value)
                        .context("announce")
                        .map(Some)?;
                }
                (b"created by", value) => {
                    created_by = String::decode_bencode_object(value)
                        .context("created_by")
                        .map(Some)?;
                }
                // (b"creation date", value) => {
                //     creation_date = u64::decode_bencode_object(value)
                //         .context("creation_date")
                //         .map(Some)?;
                // }
                // (b"httpseeds", value) => {
                //     http_seeds = Vec::decode_bencode_object(value)
                //         .context("http_seeds")
                //         .map(Some)?;
                // }
                (b"info", value) => {
                    info = Info::decode_bencode_object(value)
                        .context("info")
                        .map(Some)?;
                }
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                }
            }
        }

        let announce = announce.ok_or_else(|| Error::missing_field("announce"))?;
        let info = info.ok_or_else(|| Error::missing_field("info"))?;

        Ok(MetaInfo {
            announce,
            info,
            created_by, // comment,
                        // creation_date,
                        // http_seeds,
        })
    }
}
impl FromBencode for Info {
    const EXPECTED_RECURSION_DEPTH: usize = 1;

    /// Treats object as dictionary containing all fields for the info struct.
    /// On success the dictionary is parsed for the fields of info which are
    /// necessary for torrent. Any missing field will result in a missing field
    /// error which will stop the decoding.
    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut file_length = None;
        let mut name = None;
        let mut piece_length = None;
        let mut pieces = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"length", value) => {
                    file_length = value
                        .try_into_integer()
                        .context("file.length")
                        .map(ToString::to_string)
                        .map(Some)?;
                }
                (b"name", value) => {
                    name = String::decode_bencode_object(value)
                        .context("name")
                        .map(Some)?;
                }
                (b"piece length", value) => {
                    piece_length = value
                        .try_into_integer()
                        .context("length")
                        .map(ToString::to_string)
                        .map(Some)?;
                }
                (b"pieces", value) => {
                    pieces = AsString::decode_bencode_object(value)
                        .context("pieces")
                        .map(|bytes| Some(bytes.0))?;
                }
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                }
            }
        }

        let file_length = file_length.ok_or_else(|| Error::missing_field("file_length"))?;
        let name = name.ok_or_else(|| Error::missing_field("name"))?;
        let piece_length = piece_length.ok_or_else(|| Error::missing_field("piece_length"))?;
        let pieces = pieces.ok_or_else(|| Error::missing_field("pieces"))?;

        // Check that we discovered all necessary fields
        Ok(Info {
            file_length,
            name,
            piece_length,
            pieces,
        })
    }
}

impl FromBencode for BencodeValue {
    fn decode_bencode_object(object: Object) -> Result<Self, Error> {
        let mut dict_vec: Vec<(String, BencodeValue)> = Vec::new();
        let mut list_vec: Vec<BencodeValue> = Vec::new();
        match object {
            Object::Integer(_i) => {
                return object
                    .try_into_integer()
                    .and_then(|x| Ok(BencodeValue::Int(x.parse::<i64>().unwrap())));
            }
            Object::Bytes(_b) => {
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

pub fn decode_torrent(encoded_value: &Vec<u8>) -> Result<MetaInfo, ParseError> {
    if let Ok(val) = MetaInfo::from_bencode(encoded_value) {
        return Ok(val);
    } else {
        return Err(ParseError::InvalidFormat);
    }
}

pub fn decode_bn(encoded_value: &str) -> Result<BencodeValue, ParseError> {
    if let Ok(val) = BencodeValue::from_bencode(encoded_value.as_bytes()) {
        print_decoded_value(&val);
        return Ok(val);
    } else {
        return Err(ParseError::InvalidFormat);
    }
}
