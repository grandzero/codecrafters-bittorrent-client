// use serde_derive::Deserialize;
// #[derive(Debug, Deserialize)]
// pub struct TrackerResponseSerdeBencode {
//     #[serde(default)]
//     pub interval: Option<i32>,
//     #[serde(default)]
//     pub peers: Option<Vec<u8>>,
//     #[serde(default)]
//     pub complete: Option<i32>,
//     #[serde(default)]
//     pub incomplete: Option<i32>,
//     #[serde(default)]
//     pub min_interval: Option<i32>,
//     #[serde(default)]
//     pub _downloaded: Option<i32>,
//     #[serde(default)]
//     pub _uploaded: Option<i32>,
// }

// pub fn get_serde_bencode_tracker_response(
//     tracker_response: Vec<u8>,
// ) -> Result<TrackerResponseSerdeBencode, Box<dyn std::error::Error>> {
//     let tracker_res =
//         serde_bencode::de::from_bytes::<TrackerResponseSerdeBencode>(&tracker_response)?;
//     Ok(tracker_res)
// }

// // pub fn read_tracker_response(
// //     tracker_response: Vec<u8>,
// // ) -> Result<Vec<Peer>, Box<dyn std::error::Error>> {
// //     let peers = serde_bencode::from_bytes::<serde_bencode::value::Value>(&tracker_response)?;
// //     let peers: Vec<Peer> = match peers {
// //         Value::Dict(dict) => {
// //             println!("Dict: {:?}", dict);
// //             let peers = dict.get("peers".as_bytes()).expect("No peers found");
// //             match peers {
// //                 serde_bencode::value::Value::Bytes(b) => b
// //                     .chunks_exact(6)
// //                     .map(|chunk| Peer {
// //                         ip: format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]),
// //                         port: u16::from_be_bytes([chunk[4], chunk[5]]),
// //                     })
// //                     .collect(),
// //                 _ => panic!("Expected dict"),
// //             }
// //         }
// //         _ => panic!("Expected dict"),
// //     };
// //     println!("Peers: {:?}", peers);
// //     // let tracker_res =
// //     //     serde_bencode::de::from_bytes::<TrackerResponseSerdeBencode>(&tracker_response)?;
// //     Ok(peers)
// // }
