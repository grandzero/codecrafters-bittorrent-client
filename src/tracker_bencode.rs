use anyhow::Result;
use bendy::decoding::{Error, FromBencode, Object, ResultExt};
use bendy::encoding::AsString;
use std::fmt::{self, Display};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub struct TrackerResponse {
    interval: Option<i32>,
    peers: Option<Vec<u8>>,
    complete: Option<i32>,
    incomplete: Option<i32>,
    min_interval: Option<i32>,
}

impl FromBencode for TrackerResponse {
    const EXPECTED_RECURSION_DEPTH: usize = 1;

    /// Treats object as dictionary containing all fields for the info struct.
    /// On success the dictionary is parsed for the fields of info which are
    /// necessary for torrent. Any missing field will result in a missing field
    /// error which will stop the decoding.
    fn decode_bencode_object(object: Object) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut interval = None;
        let mut peers = None;
        let mut complete = None;
        let mut incomplete = None;
        let mut min_interval = None;

        let mut dict_dec = object.try_into_dictionary()?;
        while let Some(pair) = dict_dec.next_pair()? {
            match pair {
                (b"interval", value) => {
                    interval = i32::decode_bencode_object(value)
                        .context("interval")
                        .map(Some)?;
                }
                (b"peers", value) => {
                    peers = AsString::decode_bencode_object(value)
                        .context("peers")
                        .map(|bytes| Some(bytes.0))?;
                    // let peer_bytes = Vec::<u8>::decode_bencode_object(value)?;
                    // if let Some(val) = Some(
                    //     peer_bytes
                    //         .chunks(6)
                    //         .map(|chunk| <[u8; 6]>::try_from(chunk).unwrap())
                    //         .collect::<Vec<[u8; 6]>>(),
                    // ) {
                    //     peers = Some(val);
                    // }
                }
                (b"complete", value) => {
                    complete = i32::decode_bencode_object(value)
                        .context("complete")
                        .map(Some)?;
                }
                (b"incomplete", value) => {
                    incomplete = i32::decode_bencode_object(value)
                        .context("interval")
                        .map(Some)?;
                }
                (b"min interval", value) => {
                    min_interval = i32::decode_bencode_object(value)
                        .context("min interval")
                        .map(Some)?;
                }
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                }
            }
        }

        // let interval = interval.ok_or_else(|| Error::missing_field("interval"))?;
        // //let peers = peers.ok_or_else(|| Error::missing_field("peers"))?;
        // let complete = complete.ok_or_else(|| Error::missing_field("complete"))?;
        // let incomplete = incomplete.ok_or_else(|| Error::missing_field("incomplete"))?;
        // let min_interval = min_interval.ok_or_else(|| Error::missing_field("min interval"))?;

        Ok(TrackerResponse {
            interval,
            peers,
            complete,
            incomplete,
            min_interval,
        })
    }
}

impl Display for TrackerResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(peers) = &self.peers {
            if peers.len() % 6 != 0 {
                return Err(fmt::Error);
            } else {
                for peer in peers.chunks(6) {
                    let ip = IpAddr::V4(Ipv4Addr::new(peer[0], peer[1], peer[2], peer[3]));
                    let port = u16::from_be_bytes([peer[4], peer[5]]);
                    writeln!(f, "{}:{}", ip, port)?;
                }
            }
        } else {
            writeln!(
                f,
                "interval: {:?}, complete: {:?}, incomplete: {:?}, min_interval: {:?}",
                self.interval, self.complete, self.incomplete, self.min_interval
            )?;
        }

        Ok(())
    }
}
