use crate::app::FolderSelected;
use rfd::{AsyncFileDialog, FileDialog};
use std::{
    future::Future, path::{Path, PathBuf}, sync::mpsc::Sender
};

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left, Right
}
impl Side {
    pub fn to_selected(self, f: Folder) -> FolderSelected {
        match self {
            Self::Left => FolderSelected::Left(f),
            Self::Right => FolderSelected::Right(f)
        }
    }
}


#[derive(Debug)]
pub struct Folder {
    path: PathBuf,
}

impl Folder {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn ask(tx: Sender<Option<FolderSelected>>, side: Side) {
        let dialog = FileDialog::new()
        .set_title(
            match side {
                Side::Left => "Select Left Folder to compare",
                Side::Right => "Select Right Folder to compare"
            }
        )
        .pick_folder();

        let msg = dialog.map(|p| side.to_selected(Self::new(p)));
        tx.send(msg).unwrap(); // NOTE: unwrapped value
    }
}
