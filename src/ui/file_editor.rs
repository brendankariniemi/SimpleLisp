use std::path::PathBuf;
use egui_dock::DockState;
use crate::logic;

struct Tab {
    path: PathBuf,
    content: String,
}

struct MyTabViewer;

impl egui_dock::TabViewer for MyTabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.path.file_name().unwrap().to_string_lossy().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let screen_size = ui.max_rect();
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(screen_size.size(), egui::TextEdit::multiline(&mut tab.content));
        });
    }
}

pub struct FileTabs {
    dock_state: DockState<Tab>,
    pub(crate) open_tabs_count: usize,
}

impl FileTabs {
    pub fn new() -> Self {
        Self {
            dock_state: DockState::new(Vec::new()),
            open_tabs_count: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if self.open_tabs_count > 0 {
            egui_dock::DockArea::new(&mut self.dock_state)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut MyTabViewer);
        }
    }

    pub fn open_file(&mut self, path: PathBuf) {
        // Read file content
        match logic::file_io::read_file(&path) {
            Ok(content) => {
                // Open tab with content
                let new_tab = Tab { path, content };
                self.dock_state.push_to_focused_leaf(new_tab);
                self.open_tabs_count += 1;
            },
            Err(e) => {
                eprintln!("Error opening file {:?}: {:?}", path, e);
            }
        }
    }

    pub fn close_all_tabs(&mut self) {
        // Collect all tab indices to prevent double borrowing
        let tabs_to_remove: Vec<_> = self.dock_state.iter_all_tabs()
            .map(|((surface_index, node_index), _tab)| (surface_index, node_index, egui_dock::TabIndex(0)))
            .collect();

        // Remove the tabs
        for (surface_index, node_index, tab_index) in tabs_to_remove {
            self.dock_state.remove_tab((surface_index, node_index, tab_index));
        }

        self.open_tabs_count = 0;
    }

    pub fn save_all_files(&mut self) {
        for ((_surface_index, _node_index), tab) in self.dock_state.iter_all_tabs() {
            if let Err(e) = logic::file_io::write_file(&tab.path, &tab.content) {
                eprintln!("Error saving file {:?}: {:?}", tab.path, e);
            }
        }
    }

    pub fn save_focused_file(&mut self) {
        if let Some((_rect, tab)) = self.dock_state.find_active_focused() {
            if let Err(e) = logic::file_io::write_file(&tab.path, &tab.content) {
                eprintln!("Error saving file {:?}: {:?}", tab.path, e);
            }
        }
    }
}
