mod types;
use types::{EchoOk, GenerateOk, InitOk, Message, Payload};

type NodeID = String;

fn main() {
    // listen to stdin
    let mut input = String::new();
    let node_id = wait_for_init();
    let auto_incrementer = &mut types::AutoIncrement::new();

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
            eprintln!("unable to parse json: {}", e);
            input.clear();
            continue;
        }

        let message = msg.unwrap();
        eprintln!("message: {:?}", message);

        match message.body {
            Payload::Echo(echo) => {
                let reply = Message::new(
                    node_id.clone(),
                    message.src,
                    Payload::EchoOk(EchoOk {
                        msg_id: echo.msg_id,
                        in_reply_to: echo.msg_id,
                        echo: echo.echo,
                    }),
                );
                let reply_json = serde_json::to_string(&reply).expect("unable to serialize json");
                println!("{}", reply_json);
            }
            Payload::Generate(generate) => {
                eprintln!("generate: {:?}", generate);
                let ok_msg = GenerateOk::new(
                    generate.msg_id,
                    generate.msg_id,
                    node_id.clone(),
                    auto_incrementer.next(),
                );
                let reply = Message::new(node_id.clone(), message.src, Payload::GenerateOk(ok_msg));
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
        eprintln!("expected init message");
        panic!("expected init message");
    }
}
