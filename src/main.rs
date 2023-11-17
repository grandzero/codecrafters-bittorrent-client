mod custom_bencode_decode;
mod tcphandshake;
mod tracker;
mod tracker_bencode;
use custom_bencode_decode::decode_bn;
use custom_bencode_decode::decode_torrent;
use custom_bencode_decode::print_pieces;
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
            tracker::get_torrent_response(&info_hash, &custom_torrent.announce)?;
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

            // tracker::get_torrent_response(&info_hash, &custom_torrent.announce)
            //     // .and_then(|res| {
            //     //     if let Ok(peers) = res.peers_as_ip_and_port() {
            //     //         println!("Peers: {:?}", peers);
            //     //         return Ok(peers);
            //     //     } else {
            //     //         println!("Error: Peers could not found");
            //     //         return Err("Error: Peers could not found".into());
            //     //     }
            //     // })
            //     .and_then(|peers| {
            //         // for peer in peers {
            //         //     println!("Peer: {:?}", peer);
            //         if let Ok(peer_id) = complete_tcp_handshake_with_peer(
            //             &peer_ip, //format!("{}:{}", peer.0, peer.1).as_str(),
            //             &info_hash,
            //         ) {
            //             return Ok(peer_id);
            //         }
            //         // }
            //         return Err("Error occured while completing tcp handshake".into());
            //     })
            //     .or_else(|err| {
            //         //println!("Error: {:?}", err);
            //         return Err(err);
            //     });
        } else {
            return Err("Could not complete parsing handshake".into());
        };
    } else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}
