use crate::tracker_bencode::TrackerResponse;
use bendy::decoding::FromBencode;
use percent_encoding::percent_encode_byte;
use reqwest;
pub fn get_torrent_response(
    info_hash: &Vec<u8>,
    tracker_url: &str,
) -> Result<TrackerResponse, Box<dyn std::error::Error>> {
    let info_hash_parsed_as_url_encoding = info_hash
        .iter()
        .map(|b| percent_encode_byte(*b))
        .collect::<String>();
    let params_list = "?peer_id=12345678901234567890&port=6881&uploaded=0&downloaded=0&left=0&compact=1&info_hash=".to_owned() + &info_hash_parsed_as_url_encoding;
    let url = tracker_url.to_owned() + &params_list;
    let body = reqwest::blocking::get(&url)?;
    let body_as_text = body.bytes()?;
    println!("Body: {:?}", String::from_utf8_lossy(&body_as_text));
    let tracker_response = TrackerResponse::from_bencode(&body_as_text);
    if let Ok(res) = tracker_response {
        println!("Tracker response: {}", res);
        return Ok(res);
    } else {
        println!(
            "Error while parsing tracker response {:?}",
            tracker_response
        );
        return Err("Error".into());
    }
}
