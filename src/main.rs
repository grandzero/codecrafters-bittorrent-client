mod custom_bencode_decode;
mod protocol_message;
mod tcphandshake;
mod tracker;
mod tracker_response_serde_beconde;
use custom_bencode_decode::decode_bn;
use custom_bencode_decode::decode_torrent;
use custom_bencode_decode::print_pieces;
use protocol_message::download_file;
use std::fs;
use tcphandshake::complete_tcp_handshake_with_peer;

use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let _ = decode_bn(encoded_value);
    } else if command == "info" {
        let torrent_content_as_bytes = fs::read(&args[2]).unwrap();
        if let Ok(custom_torrent) = decode_torrent(&torrent_content_as_bytes) {
            println!("{}", custom_torrent);
            println!("Pieces Hashes: ");
            print_pieces(&custom_torrent.info.pieces);
            return Ok(());
        } else {
            println!("Error");
            return Ok(());
        }
        //println!("{:?}", decoded_value);
    } else if command == "peers" {
        let torrent_content_as_bytes = fs::read(&args[2]).unwrap();
        if let Ok(custom_torrent) = decode_torrent(&torrent_content_as_bytes) {
            let info_hash = custom_torrent.info_hash();
            tracker::get_torrent_response(
                &info_hash,
                &custom_torrent.announce,
                custom_torrent.info.length,
            )?;
        } else {
            println!("Error");
            return Ok(());
        };
    } else if command == "handshake" {
        let torrent_content_as_bytes = fs::read(&args[2]).unwrap();
        let peer_ip = &args[3];
        if let Ok(custom_torrent) = decode_torrent(&torrent_content_as_bytes) {
            let info_hash = custom_torrent.info_hash();
            if let Ok(_peer_id) = complete_tcp_handshake_with_peer(
                &peer_ip, //format!("{}:{}", peer.0, peer.1).as_str(),
                &info_hash,
            ) {
                return Ok(());
            } else {
                return Err("Error occured while completing tcp handshake".into());
            }
        }
    } else if command == "download_piece" {
        println!("Downloading piece");
        let o = &args[2];
        let mut output_path = &String::from("output.torrent");
        if o == "-o" {
            output_path = &args[3];
        }
        let torrent_file_name = &args[4];
        let piece_index = (&args[5]).parse::<i32>().unwrap();
        let torrent_content_as_bytes = fs::read(torrent_file_name).unwrap();
        let custom_torrent = decode_torrent(&torrent_content_as_bytes).unwrap();
        let info_hash: Vec<u8> = custom_torrent.info_hash();
        let result = tracker::get_torrent_response(
            &info_hash,
            &custom_torrent.announce,
            custom_torrent.info.length,
        )
        .and_then(|res| {
            if let Ok(peers) = res.peers_as_ip_and_port() {
                return Ok(peers);
            } else {
                return Err("Error: Peers could not found".into());
            }
        })
        .and_then(|peers| {
            if peers.len() == 0 {
                return Err("Error: Peers length cant be zero".into());
            }
            let peer_ip = peers[1].0.to_string() + ":" + &peers[1].1.to_string();
            complete_tcp_handshake_with_peer(&peer_ip, &info_hash)
        })
        .and_then(|mut stream| {
            download_file(
                &mut stream,
                piece_index as u32,
                custom_torrent,
                &output_path,
            )
        })
        .or_else(|err| {
            return Err(err);
        });
        return result;
    } else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}
