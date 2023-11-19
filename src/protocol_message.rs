use crate::custom_bencode_decode::MetaInfo;
use sha1::{Digest, Sha1};
use std::{io::Read, io::Write, net::TcpStream};
#[derive(Debug, Clone)]
pub struct ProtocolMessage {
    pub length: u32,
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
        let length = response.len() as u32 + 4;
        let message_type = ProtocolMessageType::from_u8(response[0])?;
        let payload = response[1..].to_vec();
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

pub fn get_block(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut length_buf: [u8; 4] = [0; 4];
    let mut test_buf: [u8; 1024] = [0; 1024];
    println!("Before read length");
    stream.read(&mut length_buf)?;
    println!("After read length: {:?}", length_buf);
    let length = u32::from_be_bytes([length_buf[0], length_buf[1], length_buf[2], length_buf[3]]);
    println!("Length: {}", length);
    let mut buf = vec![0; length as usize];
    if length > 0 {
        println!("Before read");
        stream.read(&mut buf)?;
        println!("After read: {:?}", buf);
    } else {
        stream.read(&mut test_buf)?;
        return Ok(test_buf.to_vec());
    }

    Ok(buf)
}

pub fn download_file(
    stream: &mut TcpStream,
    piece_index: u32,
    custom_torrent: MetaInfo,
    output_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Entered download file");
    let bitfield = get_block(stream)?;
    println!("Bitfield: {:?}", bitfield);
    ProtocolMessage::deserialize(&bitfield)?;

    let interested = ProtocolMessage {
        length: 4 + 1 + 1,
        message_type: ProtocolMessageType::Interested,
        payload: vec![0x00],
    };
    let serialized_interested = interested.serialize();
    stream.write(&serialized_interested)?;

    let unchoke = get_block(stream)?;
    let response = ProtocolMessage::deserialize(&unchoke.to_vec())?;
    println!("Response Unchoke: {:?}", response);

    println!("Worked until here");
    let serialized_index = piece_index.to_be_bytes();
    let piece_length = custom_torrent.info.piece_length as u32;
    let mut length: u32 = 2_u32.pow(14);
    let mut total_length: u32 = 0;
    let mut index: u32 = 0;
    let last_block_length: u32 = piece_length % length;
    let last_block_index: u32 = (piece_length / length) + 1;
    let mut piece_vector: Vec<u8> = Vec::new();
    let mut payload: Vec<u8> = Vec::new();
    payload.extend(serialized_index);
    payload.extend(total_length.to_be_bytes());
    payload.extend(length.to_be_bytes());
    println!("Payload: {:?}", payload);
    let request = ProtocolMessage {
        length: 4 + 1 + payload.len() as u32,
        message_type: ProtocolMessageType::Request,
        payload: payload,
    };
    let serialized_request = request.serialize();

    stream.write(&serialized_request)?;
    // Read response(piece)
    println!("Before reading piece");
    let piece_response = get_block(stream)?;
    println!("After reading piece response {:?}", piece_response);
    index += 1;
    total_length += length;
    while piece_length >= total_length {
        if index == last_block_index {
            length = last_block_length;
        }

        println!("Reading blocks");

        let mut payload: Vec<u8> = Vec::new();
        payload.extend(serialized_index);
        payload.extend(total_length.to_be_bytes());
        payload.extend(length.to_be_bytes());
        println!("Payload: {:?}", payload);
        let request = ProtocolMessage {
            length: 4 + 1 + payload.len() as u32,
            message_type: ProtocolMessageType::Request,
            payload: payload,
        };

        let serialized_request = request.serialize();

        stream.write(&serialized_request)?;
        // Read response(piece)
        println!("Before reading piece");
        let piece_response = get_block(stream)?;
        println!("After reading piece response {:?}", piece_response);
        // Payload includes block
        if piece_response.len() == 0 {
            println!("Piece response {:?}", piece_response);
            break;
        }
        let response = ProtocolMessage::deserialize(&piece_response.to_vec())?;
        println!("Response Piece: {:?}", response);

        // If correct merge into piece vector
        piece_vector.extend(&response.payload);
        index += 1;
        total_length += length;
    }

    println!("Piece vector completed");
    let mut hasher = Sha1::new();
    hasher.update(&piece_vector);
    let result = hasher.finalize();
    if let Some(p_hash) = custom_torrent
        .info
        .pieces
        .chunks(20)
        .nth(piece_index as usize)
    {
        println!(
            "Piece hash: {:?}, received result hash: {:?}",
            p_hash, result
        );
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
    // Write piece vector to file
    let mut file = std::fs::File::create(output_file_path)?;
    file.write_all(&piece_vector)?;
    Ok(())
}
