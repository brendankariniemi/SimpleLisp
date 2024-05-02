use crate::ui::modals::NewProjectModal;
use crate::ui::file_editor::FileTabs;
use crate::ui::project_tree::FileTree;
use crate::ui::menu;

pub struct MyApp {
    file_tabs: FileTabs,
    file_tree: FileTree,
    new_project_modal: NewProjectModal,

}

impl MyApp {
    pub fn new() -> Self {
        Self {
            file_tabs: FileTabs::new(),
            file_tree: FileTree::new(),
            new_project_modal: NewProjectModal::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::menu_bar_ui(ui, &mut self.file_tree, &mut self.file_tabs, &mut self.new_project_modal);
        });

        egui::SidePanel::left("file_tree").show(ctx, |ui| {
            self.file_tree.ui(ui, &mut self.file_tabs)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.file_tabs.ui(ui);
        });

        self.new_project_modal.show(ctx, &mut self.file_tree);
    }
}
