use crate::logic::file_io;
use crate::ui::project_tree::FileTree;


pub struct NewProjectModal {
    pub is_open: bool,
    pub location: String,
}

impl NewProjectModal {
    pub fn new() -> Self {
        Self {
            is_open: false,
            location: String::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, file_tree: &mut FileTree) {
        if self.is_open {
            let location = &mut self.location;

            egui::Window::new("New Project")
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.text_edit_singleline(location);

                    if ui.button("Create").clicked() {
                        match file_io::create_directory(location) {
                            Ok(created_path) => {
                                file_tree.set_root_directory(Some(created_path));
                                self.is_open = false;
                            },
                            Err(e) => {
                                eprintln!("Error creating directory: {}", e);
                            }
                        }
                    }
                });
        }
    }
}
