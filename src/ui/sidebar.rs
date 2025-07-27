use eframe::egui;
use std::collections::HashMap;
use crate::tools::ToolCategory;

pub struct Sidebar {
    search_query: String,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, selected_tool: &Option<ToolCategory>, tool_visibility: &HashMap<ToolCategory, bool>) -> Option<ToolCategory> {
        let mut new_selection = None;

        ui.heading("🧰 OhMyToolboxs");
        ui.separator();

        // Search box
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);
        });
        
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);

        // Tool categories
        ui.label("Tool Categories:");
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for category in ToolCategory::all() {
                // Check if this category is visible
                let is_visible = tool_visibility.get(&category).copied().unwrap_or(true);
                if !is_visible {
                    continue;
                }
                
                let is_selected = selected_tool.map_or(false, |selected| selected == category);
                
                // Filter by search query
                if !self.search_query.is_empty() && 
                   !category.name().to_lowercase().contains(&self.search_query.to_lowercase()) &&
                   !category.description().to_lowercase().contains(&self.search_query.to_lowercase()) {
                    continue;
                }

                ui.group(|ui| {
                    let response = ui.selectable_label(is_selected, format!("{} {}", category.icon(), category.name()));
                    
                    if response.clicked() {
                        new_selection = Some(category);
                    }
                    
                    // Show description on hover
                    if response.hovered() {
                        response.on_hover_text(category.description());
                    }
                    
                    // Show description below if selected
                    if is_selected {
                        ui.add_space(2.0);
                        ui.label(egui::RichText::new(category.description()).small().weak());
                    }
                });

                ui.add_space(2.0);
            }
        });

        // Footer with version info
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);
            
            let version = env!("APP_VERSION");
            let git_hash = env!("GIT_HASH");
            if git_hash != "unknown" && git_hash.len() >= 7 {
                ui.label(egui::RichText::new(format!("v{} ({})", version, &git_hash[..7])).small().weak());
            } else {
                ui.label(egui::RichText::new(format!("v{}", version)).small().weak());
            }
            
            ui.label(egui::RichText::new("Built with 🦀 Rust + egui").small().weak());
            ui.label(egui::RichText::new(format!("Built: {}", env!("BUILD_TIMESTAMP"))).small().weak());
        });

        new_selection
    }
}
