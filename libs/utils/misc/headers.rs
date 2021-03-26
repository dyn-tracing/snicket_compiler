// TODO: All of these functions should be in utils
use crate::graph::serde::FerriedData;
use std::fmt;

#[repr(i64)]
#[derive(Debug, PartialEq)]
pub enum TrafficDirection {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
}
impl From<i64> for TrafficDirection {
    fn from(orig: i64) -> Self {
        match orig {
            0x1 => return TrafficDirection::Inbound,
            0x2 => return TrafficDirection::Outbound,
            _ => return TrafficDirection::Unspecified,
        };
    }
}

impl fmt::Display for TrafficDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrafficDirection::Unspecified => write!(f, "unspecified"),
            TrafficDirection::Inbound => write!(f, "inbound"),
            TrafficDirection::Outbound => write!(f, "outbound"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum HttpType {
    Unspecified = 0,
    Request = 1,
    Response = 2,
}

pub fn data_to_str(stored_data: &FerriedData) -> Option<String> {
    let stored_data_str: String;
    match serde_json::to_string(&stored_data) {
        Ok(stored_data_str_) => {
            stored_data_str = stored_data_str_;
        }
        Err(e) => {
            log::error!("Could not translate stored data to json string: {0}\n", e);
            return None;
        }
    }
    return Some(stored_data_str);
}

pub fn join_str(str_vec: &Vec<&str>) -> String {
    return str_vec.join(".");
}
