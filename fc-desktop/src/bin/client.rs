use fc_desktop::app::FcApp;

fn main() -> eframe::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1300.0, 900.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "fc-desktop",
        native_options,
        Box::new(|_| Box::<FcApp>::default()),
    )
}
