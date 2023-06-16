use std::collections::HashMap;

use crate::types::{
    Broadcast, BroadcastOk, Echo, EchoOk, Generate, GenerateOk, Message, Payload, ReadOk, Topology,
    TopologyOk,
};

pub fn handle_echo(src: &str, dest: &str, echo: Echo) -> String {
    let reply = Message::new(
        src.to_string(),
        dest.to_string(),
        Payload::EchoOk(EchoOk {
            msg_id: echo.msg_id,
            in_reply_to: echo.msg_id,
            echo: echo.echo,
        }),
    );
    serde_json::to_string(&reply).expect("unable to serialize json")
}

pub fn handle_generate(
    node_id: &str,
    src: &str,
    dest: &str,
    luid: usize,
    generate: Generate,
) -> String {
    eprintln!("generate: {:?}", generate);
    let ok_msg = GenerateOk::new(generate.msg_id, generate.msg_id, node_id.to_string(), luid);
    let reply = Message::new(
        src.to_string(),
        dest.to_string(),
        Payload::GenerateOk(ok_msg),
    );
    serde_json::to_string(&reply).expect("unable to serialize json")
}

pub struct BroadcastRouter {
    pub messages: Vec<usize>,
    pub topology: HashMap<String, Vec<String>>,
}

impl Default for BroadcastRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl BroadcastRouter {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            topology: HashMap::new(),
        }
    }

    pub fn handle_broadcast(
        &mut self,
        src: &str,
        dest: &str,
        msg_id: usize,
        broadcast: Broadcast,
    ) -> String {
        self.messages.push(broadcast.message);

        let ok_msg = BroadcastOk {
            msg_id,
            in_reply_to: broadcast.msg_id,
        };

        let reply = Message::new(
            src.to_string(),
            dest.to_string(),
            Payload::BroadcastOk(ok_msg),
        );
        serde_json::to_string(&reply).expect("unable to serialize json")
    }

    pub fn handle_read(&self, src: &str, dest: &str, msg_id: usize) -> String {
        let messages = self.messages.clone();
        let ok_msg = ReadOk {
            msg_id,
            in_reply_to: msg_id,
            messages,
        };
        let reply = Message::new(src.to_string(), dest.to_string(), Payload::ReadOk(ok_msg));
        serde_json::to_string(&reply).expect("unable to serialize json")
    }

    pub fn handle_topology(
        &mut self,
        src: &str,
        dest: &str,
        msg_id: usize,
        topology: Topology,
    ) -> String {
        self.topology = topology.topology;

        let ok_msg = TopologyOk {
            msg_id,
            in_reply_to: msg_id,
        };
        let reply = Message::new(
            src.to_string(),
            dest.to_string(),
            Payload::TopologyOk(ok_msg),
        );
        serde_json::to_string(&reply).expect("unable to serialize json")
    }
}
