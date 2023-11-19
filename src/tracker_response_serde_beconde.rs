use std::{
    fmt::{self, Display},
    net::{IpAddr, Ipv4Addr},
};

use serde::{Deserialize, Serialize};
use serde_bencode::value::Value;
#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerResponseSerdeBencode {
    pub interval: Option<i32>,

    pub peers: Option<Vec<u8>>,

    pub complete: Option<i32>,

    pub incomplete: Option<i32>,

    pub min_interval: Option<i32>,
}
trait Integer {
    fn as_integer(&self) -> Option<i64>;
}
trait Bytes {
    fn as_bytes(&self) -> Option<&Vec<u8>>;
}

impl Integer for Value {
    fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Int(val) => Some(*val),
            _ => None,
        }
    }
}

impl Bytes for Value {
    fn as_bytes(&self) -> Option<&Vec<u8>> {
        match self {
            Value::Bytes(val) => Some(val),
            _ => None,
        }
    }
}

impl Display for TrackerResponseSerdeBencode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(peers) = &self.peers {
            if peers.len() % 6 != 0 {
                return Err(fmt::Error);
            } else {
                for peer in peers.chunks(6) {
                    let ip = IpAddr::V4(Ipv4Addr::new(peer[0], peer[1], peer[2], peer[3]));
                    let port = u16::from_be_bytes([peer[4], peer[5]]);

                    writeln!(f, "{}:{}", ip, port)?;

                    //writeln!(f, "{}:{}", ip, port)?;
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

impl TrackerResponseSerdeBencode {
    pub fn peers_as_ip_and_port(&self) -> Result<Vec<(IpAddr, u16)>, Box<dyn std::error::Error>> {
        if let Some(peers) = &self.peers {
            if peers.len() % 6 != 0 {
                return Err("Invalid ip and port".into());
            } else {
                let mut result: Vec<(IpAddr, u16)> = Vec::new();
                for peer in peers.chunks(6) {
                    let ip = IpAddr::V4(Ipv4Addr::new(peer[0], peer[1], peer[2], peer[3]));
                    let port = u16::from_be_bytes([peer[4], peer[5]]);
                    result.push((ip, port));
                }
                return Ok(result);
            }
        } else {
            return Err("Invalid ip and port".into());
        }
    }
}

pub fn get_serde_bencode_tracker_response(
    tracker_response: Vec<u8>,
) -> Result<TrackerResponseSerdeBencode, Box<dyn std::error::Error>> {
    let tracker_resp = serde_bencode::de::from_bytes(&tracker_response);
    let mut interval = None;
    let mut peers = None;
    let mut complete = None;
    let mut incomplete = None;
    let mut min_interval = None;
    match tracker_resp {
        Ok(tracker_res) => match tracker_res {
            Value::Dict(dict) => {
                for (key, value) in dict {
                    match &key[..] {
                        b"interval" => {
                            interval = value.as_integer().map(|x| Some(x as i32)).flatten();
                        }
                        b"peers" => {
                            peers = value.as_bytes().map(|x| Some(x.to_vec())).flatten();
                        }
                        b"complete" => {
                            complete = value.as_integer().map(|x| Some(x as i32)).flatten();
                        }
                        b"incomplete" => {
                            incomplete = value.as_integer().map(|x| Some(x as i32)).flatten();
                        }
                        b"min interval" => {
                            min_interval = value.as_integer().map(|x| Some(x as i32)).flatten();
                        }
                        _ => {
                            println!("Key: {:?}, Value: {:?}", key, value)
                        }
                    }
                }
                return Ok(TrackerResponseSerdeBencode {
                    interval,
                    peers,
                    complete,
                    incomplete,
                    min_interval,
                });
            }
            _ => println!("Not a dict"),
        },
        Err(err) => println!("Error in tracker rest: {:?}", err),
    }
    Err("Parse Error: Could not parse".into())
}
