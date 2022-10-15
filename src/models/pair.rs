use serde::{Deserialize, Serialize};

// use crate::Kvpair;
// use gamble::pb::abi::Kvpair;

// use crate::pb::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Hget {
    pub table: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pair {
    pub key: String,
    pub value: String,
}

// CommandRequest::new_hset("table4", "hello1111", "world1111111".to_string().into());
#[derive(Debug, Serialize, Deserialize)]
pub struct Hset {
    pub table: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hmget {
    pub table: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hmset {
    pub table: String,
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hdel {
    pub table: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hmdel {
    pub table: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hexist {
    pub table: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hmexist {
    pub table: String,
    pub keys: Vec<String>,
}

// impl From<Pair> for Kvpair {
//     fn from(pair: Pair) -> Kvpair {
//         Kvpair {
//             key: pair.key.to_owned(),
//             value: Some(pair.value.to_owned().into()),
//         }
//     }
// }

pub struct Hgetall {
    pub table: String,
}
