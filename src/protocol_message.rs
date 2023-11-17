use crate::custom_bencode_decode::MetaInfo;
use sha1::{Digest, Sha1};
use std::{io::Read, io::Write, net::TcpStream};
#[derive(Debug, Clone)]
pub struct ProtocolMessage {
    pub length: i32,
    pub message_type: ProtocolMessageType,
    pub payload: Vec<u8>,
}

impl ProtocolMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.length.to_be_bytes());
        result.push(self.message_type.as_u8());
        result.extend(&self.payload);
        result
    }

    pub fn deserialize(response: &Vec<u8>) -> Result<ProtocolMessage, Box<dyn std::error::Error>> {
        let length = i32::from_be_bytes([response[0], response[1], response[2], response[3]]);
        let message_type = ProtocolMessageType::from_u8(response[4])?;
        let payload = response[5..].to_vec();
        Ok(ProtocolMessage {
            length,
            message_type,
            payload,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProtocolMessageType {
    _KeepAlive,
    Unchoke,
    Interested,
    _NotInterested,
    _NotUsedInChallenge,
    _NotUsedInChallenge2,
    Request,
    Bitfield,
    Piece,
    _Port,
}

impl ProtocolMessageType {
    pub fn as_u8(self) -> u8 {
        let value = self as u8;
        value.clone()
    }

    pub fn from_u8(val: u8) -> Result<Self, Box<dyn std::error::Error>> {
        match val {
            1 => Ok(ProtocolMessageType::Unchoke),
            2 => Ok(ProtocolMessageType::Interested),
            5 => Ok(ProtocolMessageType::Bitfield),
            6 => Ok(ProtocolMessageType::Request),
            7 => Ok(ProtocolMessageType::Piece),
            _ => Err("Invalid message type".into()),
        }
    }
}

pub fn download_file(
    stream: &mut TcpStream,
    piece_index: i32,
    custom_torrent: MetaInfo,
    output_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    // Send bitfield
    let bitfield = ProtocolMessage {
        length: 4 + 1 + 1,
        message_type: ProtocolMessageType::Bitfield,
        payload: vec![0x00],
    };
    let serialized_bitfield = bitfield.serialize();
    stream.write(&serialized_bitfield)?;
    // Read response
    stream.read_exact(&mut buf)?;
    let response = ProtocolMessage::deserialize(&buf.to_vec())?;
    println!("Response: {:?}", response);
    // Send interested
    let interested = ProtocolMessage {
        length: 4 + 1,
        message_type: ProtocolMessageType::Interested,
        payload: vec![0x0],
    };
    let serialized_interested = interested.serialize();
    stream.write(&serialized_interested)?;
    // Read response
    stream.read_exact(&mut buf)?;
    let response = ProtocolMessage::deserialize(&buf.to_vec())?;
    println!("Response: {:?}", response);
    // Send unchoke
    let unchoke = ProtocolMessage {
        length: 4 + 1,
        message_type: ProtocolMessageType::Unchoke,
        payload: vec![0x0],
    };
    let serialized_unchoke = unchoke.serialize();
    stream.write(&serialized_unchoke)?;
    // Read response
    stream.read_exact(&mut buf)?;
    let response = ProtocolMessage::deserialize(&buf.to_vec())?;
    println!("Response: {:?}", response);
    // Send request
    let serialized_index = piece_index.to_be_bytes();
    let piece_length = custom_torrent.info.piece_length as u32;
    let mut length = 2_u32.pow(14);
    let mut total_length = 0;
    let mut index = 0;
    let last_block_length = piece_length % length;
    let last_block_index = (piece_length / length) + 1;
    let mut piece_vector: Vec<u8> = Vec::new();
    while piece_length >= total_length {
        if index == last_block_index {
            length = last_block_length;
        }

        let mut payload: Vec<u8> = Vec::new();
        payload.extend(serialized_index);
        payload.extend(index.to_be_bytes());
        payload.extend(length.to_be_bytes());
        let request = ProtocolMessage {
            length: 4 + 1 + payload.len() as i32,
            message_type: ProtocolMessageType::Request,
            payload: payload,
        };
        let serialized_request = request.serialize();
        stream.write(&serialized_request)?;
        // Read response(piece)
        let mut piece_response: Vec<u8> = Vec::new();
        // Read payload
        stream.read_exact(&mut piece_response)?;
        // Payload includes block
        let response = ProtocolMessage::deserialize(&piece_response.to_vec())?;
        // Check piece if hash correct
        let mut hasher = Sha1::new();
        hasher.update(&response.payload);
        let result = hasher.finalize();

        let piece_hash = &custom_torrent.info.pieces.chunks(20).nth(index as usize);
        if let Some(p_hash) = piece_hash {
            for (i, byte) in p_hash.iter().enumerate() {
                if *byte != result[i] {
                    println!("Hashes are not equal");
                    return Err("Hashes are not equal".into());
                }
            }
        } else {
            println!("Hashes are not equal");
            return Err("Hashes are not equal".into());
        }
        // If correct merge into piece vector
        piece_vector.extend(&response.payload);
        index += 1;
        total_length += length;
    }

    // Write piece vector to file
    let mut file = std::fs::File::create(output_file_path)?;
    file.write_all(&piece_vector)?;
    Ok(())
}
