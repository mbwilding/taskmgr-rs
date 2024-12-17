#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(taskmanager::TaskManagerApp::new(cc)))),
    )
}
