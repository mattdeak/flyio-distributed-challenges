use crate::types::{Echo, EchoOk, Generate, GenerateOk, Message, Payload};

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
