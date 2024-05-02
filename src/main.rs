use crate::app::MyApp;

mod app;
mod logic;
mod ui;
mod utils;
mod error;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "SimpleLisp",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    ).expect("panic!");
}

