use serde::{Deserialize, Serialize};

type NodeID = String;

#[derive(Serialize, Deserialize)]
struct EchoPayload {
    msg_id: usize,
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct EchoOk {
    msg_id: usize,
    in_reply_to: usize,
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct NodeInit {
    msg_id: usize,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct InitOk {
    in_reply_to: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Payload {
    #[serde(rename = "echo")]
    Echo(EchoPayload),
    #[serde(rename = "echo_ok")]
    EchoOk(EchoOk),
    #[serde(rename = "init")]
    Init(NodeInit),
    #[serde(rename = "init_ok")]
    InitOk(InitOk),
}

#[derive(Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Payload,
}

fn main() {
    // listen to stdin
    let mut input = String::new();
    let node_id = wait_for_init();
    eprintln!("node id: {}", node_id);

    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("unable to read line");

        // If Ok, then parse the json
        // If Err, then print the error and continue
        let msg: Result<Message, _> = serde_json::from_str(&input);

        if let Err(e) = msg {
            eprintln!("unable to parse json: {}", e);
            input.clear();
            continue;
        }

        let message = msg.unwrap();

        match message.body {
            Payload::Echo(echo) => {
                let reply = Message {
                    src: node_id.clone(),
                    dest: message.src,
                    body: Payload::EchoOk(EchoOk {
                        msg_id: echo.msg_id,
                        in_reply_to: echo.msg_id,
                        echo: echo.echo,
                    }),
                };
                let reply_json = serde_json::to_string(&reply).expect("unable to serialize json");
                println!("{}", reply_json);
            }
            _ => {
                eprintln!("unknown message type");
            }
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
            eprintln!("unable to parse json: {}", e);
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
        panic!("expected init message");
    }
}
