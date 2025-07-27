use eframe::egui::{self, RichText};
use std::collections::HashMap;

use crate::config::ConfigManager;
use crate::tools::ToolCategory;
use crate::tools::adb_tools::AdbFunction;
use crate::ui::sidebar::Sidebar;
use crate::ui::content::ContentArea;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct OhMyToolboxsApp {
    // Tool management
    selected_tool: Option<ToolCategory>,
    
    // UI state
    #[serde(skip)]
    sidebar: Sidebar,
    
    #[serde(skip)]
    content_area: ContentArea,
    
    // Configuration manager
    #[serde(skip)]
    config_manager: ConfigManager,
    
    // App settings (loaded from config)
    dark_mode: bool,
    sidebar_width: f32,
    
    // Tool visibility settings (loaded from config)
    tool_visibility: HashMap<ToolCategory, bool>,
    
    // Settings dialog state
    #[serde(skip)]
    settings_open: bool,
    
    #[serde(skip)]
    adb_settings_open: bool,
    
    #[serde(skip)]
    config_settings_open: bool,
    
    // Custom config path dialog state
    #[serde(skip)]
    show_custom_path_dialog: bool,
    
    #[serde(skip)]
    custom_config_path: String,
}

impl Default for OhMyToolboxsApp {
    fn default() -> Self {
        let config_manager = ConfigManager::new();
        let config = config_manager.get_config().clone();
        
        Self {
            selected_tool: None,
            sidebar: Sidebar::new(),
            content_area: ContentArea::new(),
            config_manager,
            dark_mode: config.app_settings.dark_mode,
            sidebar_width: config.app_settings.sidebar_width,
            tool_visibility: config.app_settings.tool_visibility,
            settings_open: false,
            adb_settings_open: false,
            config_settings_open: false,
            show_custom_path_dialog: false,
            custom_config_path: String::new(),
        }
    }
}

impl OhMyToolboxsApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: OhMyToolboxsApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        
        // Load saved configuration and apply to app state
        app.load_saved_settings();
        
        app
    }
}

impl eframe::App for OhMyToolboxsApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save to eframe storage (for window state, etc.)
        eframe::set_value(storage, eframe::APP_KEY, self);
        
        // Also save to our config file
        self.save_current_settings();
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the visual theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        self.render_top_panel(ctx);
        self.render_main_content(ctx);
        
        // Render settings dialog if open
        if self.settings_open {
            self.render_settings_dialog(ctx);
        }
        
        // Render ADB settings dialog if open
        if self.adb_settings_open {
            self.render_adb_settings_dialog(ctx);
        }
        
        // Render config settings dialog if open
        if self.config_settings_open {
            self.render_config_settings_dialog(ctx);
        }
    }
}

impl OhMyToolboxsApp {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        ui.menu_button("⚙️ Settings", |ui| {
                            if ui.button("Tool Categories").clicked() {
                                self.settings_open = true;
                                ui.close_menu();
                            }
                            
                            if ui.button("ADB Functions").clicked() {
                                self.adb_settings_open = true;
                                ui.close_menu();
                            }
                            
                            ui.separator();
                            
                            if ui.button("Configuration").clicked() {
                                self.config_settings_open = true;
                                ui.close_menu();
                            }
                        });
                        
                        ui.separator();
                        
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.menu_button("View", |ui| {
                    if ui.button(if self.dark_mode { "Light Mode" } else { "Dark Mode" }).clicked() {
                        self.dark_mode = !self.dark_mode;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // Show about dialog with build info
                        self.show_about_dialog(ui);
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let version = env!("APP_VERSION");
                    let git_hash = env!("GIT_HASH");
                    if git_hash != "unknown" {
                        ui.label(format!("{} v{} ({})", env!("APP_NAME"), version, &git_hash[..7]));
                    } else {
                        ui.label(format!("{} v{}", env!("APP_NAME"), version));
                    }
                });
            });
        });
    }

    fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(self.sidebar_width)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                self.sidebar_width = ui.available_width();
                if let Some(selected) = self.sidebar.render(ui, &self.selected_tool, &self.tool_visibility) {
                    self.selected_tool = Some(selected);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.content_area.render(ui, &self.selected_tool);
        });
    }

    fn show_about_dialog(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("About OhMyToolboxs")
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("🧰 OhMyToolboxs");
                    ui.add_space(10.0);
                    
                    ui.label(format!("Version: {}", env!("APP_VERSION")));
                    ui.label(format!("Target: {}", env!("TARGET")));
                    ui.label(format!("Profile: {}", env!("PROFILE")));
                    
                    let git_hash = env!("GIT_HASH");
                    if git_hash != "unknown" {
                        ui.label(format!("Git Hash: {}", git_hash));
                    }
                    
                    let git_branch = env!("GIT_BRANCH");
                    if git_branch != "unknown" {
                        ui.label(format!("Git Branch: {}", git_branch));
                    }
                    
                    ui.label(format!("Built: {}", env!("BUILD_TIMESTAMP")));
                    
                    ui.add_space(10.0);
                    ui.label(env!("APP_DESCRIPTION"));
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Built with");
                        ui.hyperlink_to("🦀 Rust", "https://www.rust-lang.org/");
                        ui.label("and");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    });
                });
            });
    }
    
    fn render_settings_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚙️ Settings")
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.heading("Tool Categories");
                ui.separator();
                ui.add_space(10.0);
                
                ui.label("Select which tool categories to show in the sidebar:");
                ui.add_space(10.0);
                
                // Create a sorted list of categories for consistent ordering
                let mut categories: Vec<ToolCategory> = ToolCategory::all();
                categories.sort_by_key(|cat| cat.name());
                
                for category in categories {
                    let mut is_visible = self.tool_visibility.get(&category).copied().unwrap_or(true);
                    let old_visible = is_visible;
                    
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut is_visible, "");
                        ui.label(format!("{} {}", category.icon(), category.name()));
                    });
                    
                    if is_visible != old_visible {
                        self.tool_visibility.insert(category, is_visible);
                        
                        // If the currently selected tool category is being hidden, clear selection
                        if !is_visible && self.selected_tool == Some(category) {
                            self.selected_tool = None;
                        }
                    }
                }
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Select All").clicked() {
                        for category in ToolCategory::all() {
                            self.tool_visibility.insert(category, true);
                        }
                    }
                    
                    if ui.button("Deselect All").clicked() {
                        for category in ToolCategory::all() {
                            self.tool_visibility.insert(category, false);
                        }
                        // Clear selection if all categories are hidden
                        self.selected_tool = None;
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.settings_open = false;
                        }
                    });
                });
            });
    }
    
    fn render_adb_settings_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("🤖 ADB Functions Settings")
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.heading("ADB Function Settings");
                ui.separator();
                ui.add_space(10.0);
                
                ui.label("Select which ADB functions to show in the ADB Tools:");
                ui.add_space(10.0);
                
                // Get access to ADB state
                let adb_state = self.content_area.get_adb_tools_state_mut();
                
                // Create a sorted list of ADB functions for consistent ordering
                let mut functions: Vec<AdbFunction> = AdbFunction::all();
                functions.sort_by_key(|func| func.name());
                
                for function in functions {
                    let mut is_visible = adb_state.adb_function_visibility.get(&function).copied().unwrap_or(true);
                    let old_visible = is_visible;
                    
                    let response = ui.horizontal(|ui| {
                        ui.checkbox(&mut is_visible, "");
                        ui.label(format!("{} {}", function.icon(), function.name()));
                    }).response;
                    
                    // Show description on hover
                    if response.hovered() {
                        response.on_hover_text(function.description());
                    }
                    
                    if is_visible != old_visible {
                        adb_state.adb_function_visibility.insert(function, is_visible);
                    }
                }
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Select All").clicked() {
                        let adb_state = self.content_area.get_adb_tools_state_mut();
                        for function in AdbFunction::all() {
                            adb_state.adb_function_visibility.insert(function, true);
                        }
                    }
                    
                    if ui.button("Deselect All").clicked() {
                        let adb_state = self.content_area.get_adb_tools_state_mut();
                        for function in AdbFunction::all() {
                            adb_state.adb_function_visibility.insert(function, false);
                        }
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.adb_settings_open = false;
                        }
                    });
                });
            });
    }
    
    fn render_config_settings_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("🔧 Configuration Settings")
            .resizable(true)
            .collapsible(false)
            .default_width(600.0)
            .show(ctx, |ui| {
                ui.heading("Configuration Management");
                ui.separator();
                ui.add_space(10.0);
                
                // Current config mode and location
                ui.group(|ui| {
                    ui.label(RichText::new("Current Configuration").strong());
                    ui.horizontal(|ui| {
                        ui.label("Mode:");
                        ui.code(self.config_manager.get_config_mode_description());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Location:");
                        ui.code(self.config_manager.get_config_path_str());
                    });
                });
                
                ui.add_space(10.0);
                
                // Config location mode selection
                ui.group(|ui| {
                    ui.label(RichText::new("Configuration Location").strong());
                    ui.add_space(5.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("📦 Switch to Portable Mode").on_hover_text("Store config next to executable (recommended for portable use)").clicked() {
                            if let Err(e) = self.config_manager.switch_to_portable_mode() {
                                eprintln!("Error switching to portable mode: {}", e);
                            }
                        }
                        
                        if ui.button("🏠 Switch to System Mode").on_hover_text("Store config in user's standard config directory").clicked() {
                            if let Err(e) = self.config_manager.switch_to_system_mode() {
                                eprintln!("Error switching to system mode: {}", e);
                            }
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("📁 Choose Custom Location").on_hover_text("Select a custom directory for config file").clicked() {
                            // In a real application, you would use a file dialog here
                            // For now, we'll just show a text input
                            self.show_custom_path_dialog = true;
                        }
                    });
                    
                    if self.show_custom_path_dialog {
                        ui.separator();
                        ui.label("Enter custom config file path:");
                        ui.text_edit_singleline(&mut self.custom_config_path);
                        ui.horizontal(|ui| {
                            if ui.button("✅ Apply").clicked() {
                                let custom_path = std::path::PathBuf::from(&self.custom_config_path);
                                if let Err(e) = self.config_manager.switch_to_custom_path(custom_path) {
                                    eprintln!("Error switching to custom path: {}", e);
                                }
                                self.show_custom_path_dialog = false;
                                self.custom_config_path.clear();
                            }
                            if ui.button("❌ Cancel").clicked() {
                                self.show_custom_path_dialog = false;
                                self.custom_config_path.clear();
                            }
                        });
                    }
                });
                
                ui.add_space(10.0);
                
                // Save/Load/Reset buttons
                ui.group(|ui| {
                    ui.label(RichText::new("Configuration Actions").strong());
                    ui.add_space(5.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Settings").on_hover_text("Save current settings to config file").clicked() {
                            self.save_current_settings();
                        }
                        
                        if ui.button("🔄 Load Settings").on_hover_text("Reload settings from config file").clicked() {
                            self.load_saved_settings();
                        }
                        
                        if ui.button("🗑️ Reset to Defaults").on_hover_text("Reset all settings to default values").clicked() {
                            self.reset_to_defaults();
                        }
                    });
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Information about what gets saved
                ui.group(|ui| {
                    ui.label(RichText::new("What gets saved:").strong());
                    ui.label("• App settings (theme, window size, etc.)");
                    ui.label("• Tool visibility preferences");
                    ui.label("• ADB tool settings and preferences");
                    ui.label("• All tool configurations and last used values");
                });
                
                ui.add_space(10.0);
                
                // Portable mode information
                ui.group(|ui| {
                    ui.label(RichText::new("📦 Portable Mode Benefits:").strong());
                    ui.label("• Config stored next to executable");
                    ui.label("• Perfect for USB drives and portable installations");
                    ui.label("• No dependency on user profile directories");
                    ui.label("• Easy to backup with the application");
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.config_settings_open = false;
                        }
                    });
                });
            });
    }
    
    fn save_current_settings(&mut self) {
        // Update config with current app settings
        let config = self.config_manager.get_config_mut();
        config.app_settings.dark_mode = self.dark_mode;
        config.app_settings.sidebar_width = self.sidebar_width;
        config.app_settings.tool_visibility = self.tool_visibility.clone();
        
        // Update ADB settings if available
        let adb_state = self.content_area.get_adb_tools_state();
        self.config_manager.update_from_adb_state(adb_state);
        
        // Save to file
        if let Err(e) = self.config_manager.save_config() {
            eprintln!("Error saving configuration: {}", e);
        }
    }
    
    fn load_saved_settings(&mut self) {
        // Reload config from file
        self.config_manager = ConfigManager::new();
        let config = self.config_manager.get_config();
        
        // Apply app settings
        self.dark_mode = config.app_settings.dark_mode;
        self.sidebar_width = config.app_settings.sidebar_width;
        self.tool_visibility = config.app_settings.tool_visibility.clone();
        
        // Apply ADB settings
        let adb_state = self.content_area.get_adb_tools_state_mut();
        self.config_manager.apply_to_adb_state(adb_state);
    }
    
    fn reset_to_defaults(&mut self) {
        // Reset to default configuration
        let default_config = crate::config::AppConfig::default();
        *self.config_manager.get_config_mut() = default_config.clone();
        
        // Apply default settings
        self.dark_mode = default_config.app_settings.dark_mode;
        self.sidebar_width = default_config.app_settings.sidebar_width;
        self.tool_visibility = default_config.app_settings.tool_visibility;
        
        // Apply default ADB settings
        let adb_state = self.content_area.get_adb_tools_state_mut();
        self.config_manager.apply_to_adb_state(adb_state);
    }
}
