#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

mod app;
mod theme;
mod diff;
use app::App;
use macroquad::prelude::*;

fn window() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        window_title: "Compa".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window)]
async fn main() {
    let mut app = App::default();

    loop {
        app.update_file_dialog();
        if app.file_dialog().is_none() {
            app.update();
        }
        app.draw();
        next_frame().await;
    }
}