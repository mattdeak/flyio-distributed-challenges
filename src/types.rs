use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Echo {
    pub msg_id: usize,
    pub echo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoOk {
    pub msg_id: usize,
    pub in_reply_to: usize,
    pub echo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInit {
    pub msg_id: usize,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitOk {
    pub in_reply_to: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Broadcast {
    pub msg_id: usize,
    pub message: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastOk {
    pub msg_id: usize,
    pub in_reply_to: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Generate {
    pub msg_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateOk {
    pub id: String,
    pub msg_id: usize,
    pub in_reply_to: usize,
}

impl GenerateOk {
    pub fn new(msg_id: usize, in_reply_to: usize, node_id: String, luid: usize) -> Self {
        let id = format!("{}-{}", node_id, luid);
        Self {
            msg_id,
            in_reply_to,
            id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Read {
    pub msg_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadOk {
    pub msg_id: usize,
    pub in_reply_to: usize,
    pub messages: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topology {
    pub msg_id: usize,
    pub topology: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopologyOk {
    pub msg_id: usize,
    pub in_reply_to: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Payload {
    #[serde(rename = "echo")]
    Echo(Echo),
    #[serde(rename = "echo_ok")]
    EchoOk(EchoOk),
    #[serde(rename = "init")]
    Init(NodeInit),
    #[serde(rename = "init_ok")]
    InitOk(InitOk),
    #[serde(rename = "generate")]
    Generate(Generate),
    #[serde(rename = "generate_ok")]
    GenerateOk(GenerateOk),
    #[serde(rename = "broadcast")]
    Broadcast(Broadcast),
    #[serde(rename = "broadcast_ok")]
    BroadcastOk(BroadcastOk),
    #[serde(rename = "read")]
    Read(Read),
    #[serde(rename = "read_ok")]
    ReadOk(ReadOk),
    #[serde(rename = "topology")]
    Topology(Topology),
    #[serde(rename = "topology_ok")]
    TopologyOk(TopologyOk),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Payload,
}

impl Message {
    pub fn new(src: String, dest: String, body: Payload) -> Self {
        Self { src, dest, body }
    }
}
