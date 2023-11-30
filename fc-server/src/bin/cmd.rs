use fc_command::{encode, Command};
use futures::{future, pin_mut, StreamExt};
use std::io::stdin;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let user_id = args
        .last()
        .map(|str| str.chars().last().unwrap_or('1'))
        .map(|c| c.to_digit(10).unwrap_or(1))
        .unwrap_or(1);

    log::info!("Auth with user id: {}", user_id);

    let url = url::Url::parse(format!("ws://127.0.0.1:8000/{}", user_id).as_str()).unwrap();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    log::info!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            let str = String::from_utf8(data.clone()).unwrap();
            log::info!("{}", str);
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);

        let string = String::from_utf8(buf).unwrap().replace("\r\n", "");
        let split: Vec<&str> = string.split('/').collect();

        let bin_cmd = match split[0] {
            "c" => encode(Command::CreateRoom(split[1].to_string())).unwrap(),
            "j" => encode(Command::JoinRoom(split[1].parse::<i64>().unwrap())).unwrap(),
            "l" => encode(Command::LeaveRoom(split[1].parse::<i64>().unwrap())).unwrap(),
            "s" => encode(Command::SendMessage(
                split[1].parse::<i64>().unwrap(),
                split[2].to_string().into_bytes(),
            ))
            .unwrap(),
            _ => {
                return;
            }
        };

        tx.unbounded_send(Message::Binary(bin_cmd)).unwrap();
    }
}
