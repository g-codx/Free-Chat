use futures::{future, pin_mut, StreamExt};
use std::sync::mpsc::{Receiver, Sender};
use tokio::runtime::Runtime;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub fn run(url: String, ws_sender: Sender<Message>, ws_receiver: Receiver<Message>) {
    let rt = Runtime::new().expect("Unable to create Runtime");
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

    std::thread::spawn(move || {
        rt.block_on(async {
            let url = url::Url::parse(url.as_str()).unwrap();

            tokio::task::spawn(async move {
                loop {
                    if let Ok(message) = ws_receiver.try_recv() {
                        stdin_tx.unbounded_send(message).unwrap();
                    }
                }
            });

            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            log::info!("WebSocket handshake has been successfully completed");

            let (write, read) = ws_stream.split();
            let stdin_to_ws = stdin_rx.map(Ok).forward(write);

            let ws_to_stdout = {
                read.for_each(|message| async {
                    match message {
                        Ok(msg) => {
                            ws_sender.send(msg).unwrap();
                        }
                        Err(err) => {
                            log::error!("{}", err);
                        }
                    }
                })
            };

            pin_mut!(stdin_to_ws, ws_to_stdout);
            future::select(stdin_to_ws, ws_to_stdout).await;
        })
    });
}
