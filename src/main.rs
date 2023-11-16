mod custom_bencode_decode;
mod tracker;
mod tracker_bencode;
use custom_bencode_decode::decode_bn;
use custom_bencode_decode::decode_torrent;
use custom_bencode_decode::print_pieces;
use std::fs;

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
            tracker::create_url(&info_hash, &custom_torrent.announce)?;
        } else {
            println!("Error");
            return Ok(());
        };
    } else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}
