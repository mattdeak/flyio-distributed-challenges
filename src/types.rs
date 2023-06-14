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

pub struct AutoIncrement {
    id: usize,
}

impl AutoIncrement {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn next(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    pub fn current(&self) -> usize {
        self.id
    }
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
