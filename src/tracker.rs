use crate::tracker_bencode::TrackerResponse;
use bendy::decoding::FromBencode;
use reqwest;

use urlencoding::encode_binary;
pub fn get_torrent_response(
    info_hash: &Vec<u8>,
    tracker_url: &str,
    left: i64,
) -> Result<TrackerResponse, Box<dyn std::error::Error>> {
    let info_encoded = encode_binary(info_hash);
    // thread::sleep(Duration::from_secs(60));
    let params_list = format!("?peer_id=00112233445566778899&port=6881&uploaded=0&downloaded=0&left={}&compact=1&info_hash=", left as i32) + &info_encoded;

    let url = tracker_url.to_owned() + &params_list;
    println!("Requesting tracker: {}", url);
    let response = reqwest::blocking::get(&url).expect("Failed to send request");
    let body = response.bytes().expect("Failed to get response body");
    println!("Response body: {:?}", String::from_utf8_lossy(&body));

    let tracker_response = TrackerResponse::from_bencode(&body);
    println!("Tracker response: {:?}", tracker_response);
    if let Ok(res) = tracker_response {
        println!("Tracker response: {}", res);
        return Ok(res);
    } else {
        // println!(
        //     "Error while parsing tracker response {:?}",
        //     tracker_response
        // );
        return Err("Error while parsing tracker response".into());
    }
}
