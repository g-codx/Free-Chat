use fc_desktop::app::FcApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1300.0, 900.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "fc-desktop",
        native_options,
        Box::new(|_| Box::new(FcApp::default())),
    )
}
