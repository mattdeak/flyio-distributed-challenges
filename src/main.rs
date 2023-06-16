pub mod autoincrement;
pub mod handlers;
pub mod types;
use handlers::{handle_echo, handle_generate};
use types::{EchoOk, GenerateOk, InitOk, Message, Payload};

type NodeID = String;

fn main() {
    // listen to stdin
    let mut input = String::new();
    let node_id = wait_for_init();
    let auto_incrementer = &mut autoincrement::AutoIncrement::new();

    let broadcast_router = &mut handlers::BroadcastRouter::new();

    eprintln!("node id: {}", node_id);
    eprintln!("beginning loop");

    loop {
        eprintln!("waiting for input");

        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| {
                eprintln!("unable to read line: {}", e);
                e
            })
            .expect("unable to read line");

        // If Ok, then parse the json
        // If Err, then print the error and continue
        let msg: Result<Message, _> = serde_json::from_str(&input);

        if let Err(e) = msg {
            eprintln!("unable to parse json for message {}: {}", &input, e);
            input.clear();
            continue;
        }

        let message = msg.unwrap();
        eprintln!("message: {:?}", message);

        let value = match message.body {
            Payload::Echo(echo) => Some(handle_echo(&message.dest, &message.src, echo)),
            Payload::Generate(generate) => Some(handle_generate(
                &node_id,
                &message.dest,
                &message.src,
                auto_incrementer.increment(),
                generate,
            )),
            Payload::Broadcast(broadcast) => Some(broadcast_router.handle_broadcast(
                &node_id,
                &message.src,
                auto_incrementer.increment(),
                broadcast,
            )),
            Payload::Read(read) => {
                Some(broadcast_router.handle_read(&node_id, &message.src, read.msg_id))
            }
            Payload::Topology(topology) => Some(broadcast_router.handle_topology(
                &node_id,
                &message.src,
                auto_incrementer.increment(),
                topology,
            )),
            _ => {
                eprintln!("unknown message type: {:?}", message.body);
                None
            }
        };

        if let Some(value) = value {
            eprintln!("sending: {}", value);
            println!("{}", value);
        }

        input.clear();
    }
}

fn wait_for_init() -> NodeID {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("unable to read line");

    let msg: Message = serde_json::from_str(&input)
        .map_err(|e| {
            eprintln!("unable to parse json from string {}. Err: {}", &input, e);
            e
        })
        .expect("unable to parse json");
    if let Payload::Init(init) = msg.body {
        let node_id = init.node_id;
        let response = Message {
            src: msg.dest,
            dest: msg.src,
            body: Payload::InitOk(InitOk {
                in_reply_to: init.msg_id,
            }),
        };
        let response_json = serde_json::to_string(&response).expect("unable to serialize json");
        println!("{}", response_json);
        node_id
    } else {
        eprintln!("expected init message");
        panic!("expected init message");
    }
}
