## Free-Chat

The application consists of a server and a desktop

The server engine accepts websocket and http connections (hyper + tokio-tungstenite)

The desktop application uses egui

Run server

    cargo run --bin server

Run client

    cargo run --bin client

Also, cmd client for testing

    cargo run --bin cmd