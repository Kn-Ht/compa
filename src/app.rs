use std::{future::{Future, IntoFuture}, thread::{self, JoinHandle, Thread}};

use crate::diff::folder::{Folder, Side};
use crate::theme;
use macroquad::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Default)]
pub enum State {
    #[default]
    NoneSelected,
    LeftSelected(Folder),
    RightSelected(Folder),
    AllSelected {
        left: Folder,
        right: Folder,
    },
    Indexing,
    Done,
}

impl State {
    pub fn select_right(&mut self) {
        *self = match self {
            Self::NoneSelected => Self::NoneSelected,
            _ => todo!(),
        };
    }
}

#[derive(Debug)]
pub enum FolderSelected {
    Left(Folder),
    Right(Folder),
}

pub struct DialogThread {
    thread: JoinHandle<()>,
    rx: Receiver<Option<FolderSelected>>,
}

#[derive(Default)]
pub struct App {
    state: State,
    file_dialog: Option<DialogThread>,
}

impl App {
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
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::A) {
            if self.file_dialog.is_none() {
                self.open_file_dialog(Side::Left);
            }
        }
    }
    pub fn draw(&mut self) {
        clear_background(theme::window::BG_COLOR);
        draw_text("HeLLO", 50.0, 50.0, 20.0, theme::text::COLOR);
    }
    pub const fn file_dialog(&self) -> &Option<DialogThread> {
        &self.file_dialog
    }
}
