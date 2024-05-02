use std::fs;
use std::path::{Path, PathBuf};
use egui::{CollapsingHeader, Ui};
use crate::ui::file_editor::FileTabs;

pub enum FileNode {
    File(PathBuf),
    Directory(PathBuf, Vec<FileNode>),
}

pub struct FileTree {
    root_node: Option<FileNode>,
}

impl FileTree {
    pub fn new() -> Self {
        Self { root_node: None }
    }

    pub fn set_root_directory(&mut self, path: Option<PathBuf>) {
        match path {
            Some(path_buf) => {
                self.root_node = Some(read_directory(&path_buf));
            },
            None => {
                self.root_node = None;
            },
        }
    }

    pub fn ui(&self, ui: &mut Ui, my_tabs: &mut FileTabs) {
        if let Some(root) = &self.root_node {
            show_file_node(ui, root, my_tabs, true);
        }
    }
}


// Function to read a directory and create the FileNode structure
fn read_directory(path: &Path) -> FileNode {
    let mut nodes = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                nodes.push(read_directory(&path));
            } else {
                nodes.push(FileNode::File(path));
            }
        }
    }
    FileNode::Directory(path.to_path_buf(), nodes)
}

fn show_file_node(ui: &mut Ui, node: &FileNode, my_tabs: &mut FileTabs, is_root: bool) {
    match node {
        FileNode::File(path) => {
            let label = ui.selectable_label(false, path.file_name().unwrap().to_string_lossy());
            if label.double_clicked() {
                my_tabs.open_file(path.clone());
            }
        }
        FileNode::Directory(path, children) => {
            let header = CollapsingHeader::new(path.file_name().unwrap().to_string_lossy())
                .default_open(is_root); // Open the root node by default

            let header_response = header.show(ui, |ui| {
                for child in children {
                    show_file_node(ui, child, my_tabs, false);
                }
            }).header_response;

            if header_response.secondary_clicked() {
                println!("Right-clicked on directory: {:?}", path);
            }
        }
    }
}

