mod custom_bencode_decode;
mod tracker;
mod tracker_bencode;
use custom_bencode_decode::decode_bn;
use custom_bencode_decode::decode_torrent;
use custom_bencode_decode::print_pieces;
use std::fs;
use std::io::Write;
use tracker_bencode::TrackerResponse;

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
    } else if command == "test" {
        let mut file = std::fs::File::create("test.torrent")?;
        let peers_as_bytes: Vec<[u8; 6]> = vec![
            "192".as_bytes().try_into().unwrap(),
            "168".as_bytes().try_into().unwrap(),
            "0".as_bytes().try_into().unwrap(),
            "1".as_bytes().try_into().unwrap(),
            "6881".as_bytes().try_into().unwrap(),
        ];
        let mut total_length = 0;
        for l in peers_as_bytes.iter() {
            total_length += l.len();
        }

        let except_peers =
            "d8:completei8e10:incompletei1e8:intervali1800e12:min intervali1800e5:peers".to_owned()
                + total_length.to_string().as_str()
                + &":";
        let mut text = except_peers.as_bytes().to_vec();
        for v in peers_as_bytes.iter() {
            text.append(&mut v.to_vec());
        }

        file.write_all(&text)?;
    } else {
        println!("unknown command: {}", args[1])
    }
    Ok(())
}
