use std::sync::mpsc;

fn main() -> eframe::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let (ws_sender, app_receiver) = mpsc::channel();
    let (app_sender, ws_receiver) = mpsc::channel();
    let url = format!("ws://127.0.0.1:3000/{}", 2);

    fc_desktop::ws::client::run(url, ws_sender, ws_receiver);
    fc_desktop::app::FcApp::new(app_sender, app_receiver).run()
}
