use std::{
    future::{Future, IntoFuture},
    thread::{self, JoinHandle, Thread},
};

use crate::{diff::folder::{Folder, Side}, ui};
use crate::theme;
use macroquad::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub enum FolderSelected {
    Left(Folder),
    Right(Folder),
}

pub struct DialogThread {
    thread: JoinHandle<()>,
    rx: Receiver<Option<FolderSelected>>,
}

pub struct App {
    left: Option<Folder>,
    right: Option<Folder>,
    file_dialog: Option<DialogThread>,
    folder_icon: Texture2D,
}

impl App {
    pub fn new() -> Self {
        Self {
            folder_icon: Texture2D::from_file_with_format(
                include_bytes!("../assets/folder_icon.png"),
                Some(ImageFormat::Png),
            ),
            left: None,
            right: None,
            file_dialog: None,
        }
    }
    fn open_file_dialog(&mut self, side: Side) {
        let (tx, rx) = channel();
        self.file_dialog = Some(DialogThread {
            thread: thread::spawn(move || Folder::ask(tx, side)),
            rx,
        });
    }
    pub fn update_file_dialog(&mut self) {
        if let Some(ref file_dialog) = self.file_dialog {
            if let Ok(msg) = file_dialog.rx.try_recv() {
                println!("Received: {msg:?}");
                self.file_dialog = None;
            }
        }
    }
    #[inline(always)]
    fn folder_params(&self, scaled_size: f32) -> DrawTextureParams {
        DrawTextureParams {
            flip_x: false,
            flip_y: false,
            source: None,
            rotation: 0.0,
            pivot: None,
            dest_size: Some(Vec2::new(scaled_size, scaled_size))
        }
    }
    pub fn update(&mut self) {}
    pub fn draw(&mut self) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        let screen_center = screen_size / 2.0;
        let scaling = 0.2;
        let scaled_w = screen_size.x * scaling;
        let font_size = scaled_w / 7.6;

        clear_background(theme::window::BG_COLOR);

        draw_line(
            screen_center.x,
            0.0,
            screen_center.x,
            screen_size.y,
            2.0,
            theme::window::SEP_COLOR,
        );

        if self.left.is_none() {
            let x = screen_center.x / 2.0 - scaled_w / 2.0; // NOTE: possible optim. (a-b)/2
            let y = screen_center.y - scaled_w / 2.0;
            draw_texture_ex(&self.folder_icon, x, y, WHITE, self.folder_params(scaled_w));
            draw_text("No folder selected.", x, y + scaled_w, font_size, theme::text::SUBTLE_COLOR);
        }
        if self.right.is_none() {
            let x = screen_center.x * 1.5 - scaled_w / 2.0;
            let y = screen_center.y - scaled_w / 2.0;
            draw_texture_ex(&self.folder_icon, x, y, WHITE, self.folder_params(scaled_w));
            draw_text("No folder selected.", x, y + scaled_w, font_size, theme::text::SUBTLE_COLOR);
        }

        ui::button("Hello, World!", Rect::new(50.0, 50.0, 200.0, 50.0), font_size);

        //draw_texture(&self.folder_icon, 200.0, 200.0, WHITE);
        //draw_texture_ex(&self.folder_icon, 200.0, 200.0, WHITE, folder_params);

        if cfg!(debug_assertions) {
            draw_text(&format!("FPS: {}", get_fps()), 0.0, 20.0, 20.0, GREEN);
        }
    }
    pub const fn file_dialog(&self) -> &Option<DialogThread> {
        &self.file_dialog
    }
}
