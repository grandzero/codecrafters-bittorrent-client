use std::convert::From;
use std::io::{Read, Write};
use std::net::TcpStream;
#[derive(Debug, Clone)]
pub struct TcpHandshake {
    pub length: u8,
    pub protocol: Vec<u8>,
    pub reserved_bytes: Vec<u8>,
    pub info_hash: Vec<u8>,
    pub peer_id: Vec<u8>,
}

pub enum TcpHandshakeError {
    InvalidLength,
    TcpStreamError(std::io::Error),
}

impl From<std::io::Error> for TcpHandshakeError {
    fn from(error: std::io::Error) -> Self {
        TcpHandshakeError::TcpStreamError(error)
    }
}

fn serialize_handshake(handshake: TcpHandshake) -> Vec<u8> {
    let mut result = Vec::new();
    result.push(handshake.length);
    result.extend(handshake.protocol);
    result.extend(handshake.reserved_bytes);
    result.extend(handshake.info_hash);
    result.extend(handshake.peer_id);
    result
}
// fn check_zero_bytes(pattern: u8) -> bool {
//     let check: &[u8] = &[115];

//     //println!("patternlen: {:?}", pattern.len());
//     if pattern == check[0] && pattern != b's' {
//         return true;
//     }
//     false
// }

fn create_str_from_hex(hex: &Vec<u8>) -> String {
    let mut result = String::new();
    for byte in hex {
        result.push_str(&format!("{:x}", byte));
    }
    result
}
pub fn complete_tcp_handshake_with_peer(
    ip_address: &str,
    info_hash: &Vec<u8>,
) -> Result<Vec<u8>, TcpHandshakeError> {
    println!("Connecting to: {}", ip_address);
    let handshake = TcpHandshake {
        length: 19,
        protocol: b"BitTorrent protocol".to_vec(),
        reserved_bytes: vec![0; 8],
        info_hash: info_hash.to_vec(),
        peer_id: b"00112233445566778899".to_vec(),
    };
    let len = serialize_handshake(handshake.clone()).len();

    if len != 68 {
        return Err(TcpHandshakeError::InvalidLength);
    }
    let mut stream = TcpStream::connect(ip_address)?;
    stream.write_all(&serialize_handshake(handshake))?;
    let mut buffer = [0; 68];
    stream.read(&mut buffer)?;

    println!("Handshake successful");
    println!("Peer ID: {}", create_str_from_hex(&(buffer[48..]).to_vec()));
    return Ok((&buffer[48..]).to_vec());
}
