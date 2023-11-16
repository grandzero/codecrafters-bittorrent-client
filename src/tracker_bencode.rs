use anyhow::Result;

use bendy::{
    decoding::{Error, FromBencode, Object, ResultExt},
    encoding::AsString,
};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct TrackerResponse {
    interval: Option<i32>,
    peers: Option<Vec<u8>>,
    complete: Option<i32>,
    incomplete: Option<i32>,
    min_interval: Option<i32>,
    _downloaded: Option<i32>,
    _uploaded: Option<i32>,
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
        let mut downloaded = None;
        let mut uploaded = None;

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
                }
                (b"min interval", value) => {
                    min_interval = i32::decode_bencode_object(value)
                        .context("min interval")
                        .map(Some)?;
                }
                (b"complete", value) => {
                    complete = i32::decode_bencode_object(value)
                        .context("complete")
                        .map(Some)?;
                }
                (b"incomplete", value) => {
                    incomplete = i32::decode_bencode_object(value)
                        .context("incomplete")
                        .map(Some)?;
                }

                (b"downloaded", value) => {
                    downloaded = i32::decode_bencode_object(value)
                        .context("downloaded")
                        .map(Some)?;
                }
                (b"uploaded", value) => {
                    uploaded = i32::decode_bencode_object(value)
                        .context("uploaded")
                        .map(Some)?;
                }
                (unknown_field, _) => {
                    return Err(Error::unexpected_field(String::from_utf8_lossy(
                        unknown_field,
                    )));
                }
            }
        }

        Ok(TrackerResponse {
            interval,
            peers,
            complete,
            incomplete,
            min_interval,
            _downloaded: downloaded,
            _uploaded: uploaded,
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
                    writeln!(
                        f,
                        "{}.{}.{}.{}:{}{}",
                        peer[0], peer[1], peer[2], peer[3], peer[4], peer[5]
                    )?;
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
