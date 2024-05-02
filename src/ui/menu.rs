use egui::Ui;
use rfd::FileDialog;
use crate::ui::file_editor::FileTabs;
use crate::ui::modals::NewProjectModal;
use crate::ui::project_tree::FileTree;


pub fn menu_bar_ui(ui: &mut Ui, file_tree: &mut FileTree, file_tabs: &mut FileTabs, new_project_modal: &mut NewProjectModal) {

    egui::menu::bar(ui, |ui| {
        let mut close_menu = false;

        ui.menu_button("File", |ui| {
            if ui.button("New Project").clicked() {
                // TODO: Add (Do you want to save?) popup
                file_tree.set_root_directory(None);
                file_tabs.close_all_tabs();
                new_project_modal.is_open = true;
                close_menu = true;
            }
            if ui.button("Open Project").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    file_tree.set_root_directory(Some(path));
                }
                close_menu = true;
            }
            if ui.button("Close Project").clicked() {
                // TODO: Add (Do you want to save?) popup
                file_tree.set_root_directory(None);
                file_tabs.close_all_tabs();
                close_menu = true;
            }
            if ui.button("Save").clicked() {
                file_tabs.save_focused_file();
                close_menu = true;
            }
            if ui.button("Save All").clicked() {
                file_tabs.save_all_files();
                close_menu = true;
            }

            if close_menu {
                ui.close_menu();
            }
        });

        ui.menu_button("Edit", |ui| {
            let mut close_menu = false;

            if ui.button("Undo").clicked() {
                // TODO: Add undo action
                println!("Undo clicked");
                close_menu = true;
            }
            if ui.button("Redo").clicked() {
                // TODO: Add redo action
                println!("Redo clicked");
                close_menu = true;
            }
            if ui.button("Find").clicked() {
                // TODO: Add find action
                println!("Find clicked");
                close_menu = true;
            }
            if ui.button("Select All").clicked() {
                // TODO: Add select all action
                println!("Select All clicked");
                close_menu = true;
            }

            if close_menu {
                ui.close_menu();
            }
        });
    });
}
