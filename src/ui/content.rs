use eframe::egui;
use crate::tools::ToolCategory;
use crate::tools::adb_tools::{AdbToolsState, show_adb_tools};

pub struct ContentArea {
    adb_tools: AdbToolsState,
}

impl ContentArea {
    pub fn new() -> Self {
        Self {
            adb_tools: AdbToolsState::default(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, selected_tool: &Option<ToolCategory>) {
        match selected_tool {
            Some(ToolCategory::AdbTools) => {
                show_adb_tools(ui, &mut self.adb_tools);
            }
            None => {
                self.render_welcome(ui);
            }
        }
    }

    fn render_welcome(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            ui.heading("🧰 Welcome to OhMyToolboxs");
            ui.add_space(20.0);
            
            ui.label("A comprehensive desktop toolbox application built with Rust and egui");
            ui.add_space(30.0);
            
            ui.group(|ui| {
                ui.set_min_width(400.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Available Tool Categories:");
                    ui.add_space(10.0);
                    
                    for category in ToolCategory::all() {
                        ui.horizontal(|ui| {
                            ui.label(category.icon());
                            ui.strong(category.name());
                            ui.label("-");
                            ui.label(category.description());
                        });
                        ui.add_space(5.0);
                    }
                });
            });
            
            ui.add_space(30.0);
            ui.label("👈 Select a tool category from the sidebar to get started");
            
            ui.add_space(50.0);
            
            // Quick stats or tips
            ui.group(|ui| {
                ui.set_min_width(400.0);
                ui.vertical_centered(|ui| {
                    ui.heading("💡 Tips");
                    ui.add_space(10.0);
                    
                    ui.label("• Use the search box in the sidebar to quickly find tools");
                    ui.label("• All tools work offline and don't send data to external servers");
                    ui.label("• Your preferences are automatically saved");
                    ui.label("• Use Ctrl+C to copy results to clipboard");
                });
            });
        });
    }

    pub fn get_adb_tools_state_mut(&mut self) -> &mut AdbToolsState {
        &mut self.adb_tools
    }

    pub fn get_adb_tools_state(&self) -> &AdbToolsState {
        &self.adb_tools
    }
}
