use eframe::egui;
use crate::tools::ToolCategory;
use crate::tools::adb_tools::{AdbToolsState, show_adb_tools};
use crate::tools::fastboot_tools::{FastbootToolsState, show_fastboot_tools};
use crate::tools::qdl_tools::{QdlToolsState, show_qdl_tools};
use crate::tools::qramdump_tools::{QramdumpToolsState, show_qramdump_tools};

pub struct ContentArea {
    adb_tools: AdbToolsState,
    fastboot_tools: FastbootToolsState,
    qdl_tools: QdlToolsState,
    qramdump_tools: QramdumpToolsState,
}

impl ContentArea {    pub fn new() -> Self {
        Self {
            adb_tools: AdbToolsState::default(),
            fastboot_tools: FastbootToolsState::default(),
            qdl_tools: QdlToolsState::default(),
            qramdump_tools: QramdumpToolsState::default(),
        }
    }    pub fn render(&mut self, ui: &mut egui::Ui, selected_tool: &Option<ToolCategory>) {
        match selected_tool {
            Some(ToolCategory::AdbTools) => {
                show_adb_tools(ui, &mut self.adb_tools);
            }
            Some(ToolCategory::FastbootTools) => {
                show_fastboot_tools(ui, &mut self.fastboot_tools);
            }
            Some(ToolCategory::QdlTools) => {
                show_qdl_tools(ui, &mut self.qdl_tools);
            }
            Some(ToolCategory::QramdumpTools) => {
                show_qramdump_tools(ui, &mut self.qramdump_tools);
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
    }    pub fn get_adb_tools_state_mut(&mut self) -> &mut AdbToolsState {
        &mut self.adb_tools
    }

    pub fn get_adb_tools_state(&self) -> &AdbToolsState {
        &self.adb_tools
    }

    pub fn get_fastboot_tools_state_mut(&mut self) -> &mut FastbootToolsState {
        &mut self.fastboot_tools
    }    pub fn get_fastboot_tools_state(&self) -> &FastbootToolsState {
        &self.fastboot_tools
    }

    pub fn get_qdl_tools_state_mut(&mut self) -> &mut QdlToolsState {
        &mut self.qdl_tools
    }

    pub fn get_qdl_tools_state(&self) -> &QdlToolsState {
        &self.qdl_tools
    }

    pub fn get_qramdump_tools_state_mut(&mut self) -> &mut QramdumpToolsState {
        &mut self.qramdump_tools
    }

    pub fn get_qramdump_tools_state(&self) -> &QramdumpToolsState {
        &self.qramdump_tools
    }
}
