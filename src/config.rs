use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::tools::ToolCategory;
use crate::tools::adb_tools::{AdbFunction, AdbToolsState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_settings: AppSettings,
    pub tool_settings: ToolSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub dark_mode: bool,
    pub sidebar_width: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub tool_visibility: HashMap<ToolCategory, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSettings {
    pub adb_tools: AdbToolsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdbToolsConfig {
    pub selected_device: Option<String>,
    pub package_filter: String,
    pub apk_path: String,
    pub local_path: String,
    pub remote_path: String,
    pub shell_command: String,
    pub logcat_filter: String,
    pub screenshot_path: String,
    pub screen_record_path: String,
    pub local_port: String,
    pub remote_port: String,
    pub monitor_interval: f32,
    pub show_plots: bool,
    pub adb_function_visibility: HashMap<AdbFunction, bool>,
    pub selinux_file_path: String,
    pub selinux_new_context: String,
    pub systemd_service_name: String,
    pub systemd_unit_filter: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_settings: AppSettings::default(),
            tool_settings: ToolSettings::default(),
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        let mut tool_visibility = HashMap::new();
        for category in ToolCategory::all() {
            tool_visibility.insert(category, true);
        }
        
        Self {
            dark_mode: true,
            sidebar_width: 250.0,
            window_width: 1200.0,
            window_height: 800.0,
            tool_visibility,
        }
    }
}

impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            adb_tools: AdbToolsConfig::default(),
        }
    }
}

impl Default for AdbToolsConfig {
    fn default() -> Self {
        let mut adb_function_visibility = HashMap::new();
        for function in AdbFunction::all() {
            adb_function_visibility.insert(function, true);
        }
        
        Self {
            selected_device: None,
            package_filter: String::new(),
            apk_path: String::new(),
            local_path: String::new(),
            remote_path: String::new(),
            shell_command: String::new(),
            logcat_filter: String::new(),
            screenshot_path: "screenshot.png".to_string(),
            screen_record_path: "recording.mp4".to_string(),
            local_port: "8080".to_string(),
            remote_port: "8080".to_string(),
            monitor_interval: 1.0,
            show_plots: true,
            adb_function_visibility,
            selinux_file_path: String::new(),
            selinux_new_context: String::new(),
            systemd_service_name: String::new(),
            systemd_unit_filter: String::new(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: AppConfig,
    use_portable_mode: bool,
}

impl ConfigManager {
    pub fn new() -> Self {
        let (config_path, use_portable_mode) = Self::determine_config_path();
        let config = Self::load_config(&config_path);
        
        Self {
            config_path,
            config,
            use_portable_mode,
        }
    }
    
    pub fn new_with_custom_path(custom_path: PathBuf) -> Self {
        let config = Self::load_config(&custom_path);
        
        Self {
            config_path: custom_path,
            config,
            use_portable_mode: false,
        }
    }
    
    fn determine_config_path() -> (PathBuf, bool) {
        // First, try portable mode (config next to executable)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let portable_config = exe_dir.join("config.toml");
                
                // Check if portable config already exists
                if portable_config.exists() {
                    return (portable_config, true);
                }
                
                // Check if we can write to the executable directory (portable mode)
                let test_file = exe_dir.join(".write_test");
                if fs::write(&test_file, "test").is_ok() {
                    let _ = fs::remove_file(&test_file);
                    return (portable_config, true);
                }
            }
        }
        
        // Fall back to system config directory
        (Self::get_system_config_path(), false)
    }
    
    fn get_system_config_path() -> PathBuf {
        let mut path = dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap());
        path.push("ohmytoolboxs");
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        path.push("config.toml");
        path
    }
    
    fn get_portable_config_path() -> Option<PathBuf> {
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                return Some(exe_dir.join("config.toml"));
            }
        }
        None
    }
    
    fn load_config(path: &PathBuf) -> AppConfig {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match toml::from_str(&content) {
                        Ok(config) => {
                            println!("✅ Configuration loaded from {:?}", path);
                            return config;
                        }
                        Err(e) => {
                            println!("⚠️ Error parsing config file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️ Error reading config file: {}", e);
                }
            }
        }
        
        println!("📝 Using default configuration");
        AppConfig::default()
    }
    
    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        println!("💾 Configuration saved to {:?}", self.config_path);
        Ok(())
    }
    
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }
    
    pub fn get_config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }
    
    pub fn update_from_adb_state(&mut self, adb_state: &AdbToolsState) {
        let adb_config = &mut self.config.tool_settings.adb_tools;
        
        adb_config.selected_device = adb_state.selected_device.clone();
        adb_config.package_filter = adb_state.package_filter.clone();
        adb_config.apk_path = adb_state.apk_path.clone();
        adb_config.local_path = adb_state.local_path.clone();
        adb_config.remote_path = adb_state.remote_path.clone();
        adb_config.shell_command = adb_state.shell_command.clone();
        adb_config.logcat_filter = adb_state.logcat_filter.clone();
        adb_config.screenshot_path = adb_state.screenshot_path.clone();
        adb_config.screen_record_path = adb_state.screen_record_path.clone();
        adb_config.local_port = adb_state.local_port.clone();
        adb_config.remote_port = adb_state.remote_port.clone();
        adb_config.monitor_interval = adb_state.monitor_interval;
        adb_config.show_plots = adb_state.show_plots;
        adb_config.adb_function_visibility = adb_state.adb_function_visibility.clone();
        adb_config.selinux_file_path = adb_state.selinux_file_path.clone();
        adb_config.selinux_new_context = adb_state.selinux_new_context.clone();
        adb_config.systemd_service_name = adb_state.systemd_service_name.clone();
        adb_config.systemd_unit_filter = adb_state.systemd_unit_filter.clone();
    }
    
    pub fn apply_to_adb_state(&self, adb_state: &mut AdbToolsState) {
        let adb_config = &self.config.tool_settings.adb_tools;
        
        adb_state.selected_device = adb_config.selected_device.clone();
        adb_state.package_filter = adb_config.package_filter.clone();
        adb_state.apk_path = adb_config.apk_path.clone();
        adb_state.local_path = adb_config.local_path.clone();
        adb_state.remote_path = adb_config.remote_path.clone();
        adb_state.shell_command = adb_config.shell_command.clone();
        adb_state.logcat_filter = adb_config.logcat_filter.clone();
        adb_state.screenshot_path = adb_config.screenshot_path.clone();
        adb_state.screen_record_path = adb_config.screen_record_path.clone();
        adb_state.local_port = adb_config.local_port.clone();
        adb_state.remote_port = adb_config.remote_port.clone();
        adb_state.monitor_interval = adb_config.monitor_interval;
        adb_state.show_plots = adb_config.show_plots;
        adb_state.adb_function_visibility = adb_config.adb_function_visibility.clone();
        adb_state.selinux_file_path = adb_config.selinux_file_path.clone();
        adb_state.selinux_new_context = adb_config.selinux_new_context.clone();
        adb_state.systemd_service_name = adb_config.systemd_service_name.clone();
        adb_state.systemd_unit_filter = adb_config.systemd_unit_filter.clone();
    }
    
    pub fn get_config_path_str(&self) -> String {
        self.config_path.to_string_lossy().to_string()
    }
    
    pub fn is_portable_mode(&self) -> bool {
        self.use_portable_mode
    }
    
    pub fn switch_to_portable_mode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(portable_path) = Self::get_portable_config_path() {
            // Save current config to portable location
            let content = toml::to_string_pretty(&self.config)?;
            fs::write(&portable_path, content)?;
            
            self.config_path = portable_path;
            self.use_portable_mode = true;
            println!("📦 Switched to portable mode: {:?}", self.config_path);
            Ok(())
        } else {
            Err("Cannot determine executable location for portable mode".into())
        }
    }
    
    pub fn switch_to_system_mode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let system_path = Self::get_system_config_path();
        
        // Save current config to system location
        let content = toml::to_string_pretty(&self.config)?;
        if let Some(parent) = system_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&system_path, content)?;
        
        self.config_path = system_path;
        self.use_portable_mode = false;
        println!("🏠 Switched to system mode: {:?}", self.config_path);
        Ok(())
    }
    
    pub fn switch_to_custom_path(&mut self, custom_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Save current config to custom location
        let content = toml::to_string_pretty(&self.config)?;
        if let Some(parent) = custom_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&custom_path, content)?;
        
        self.config_path = custom_path;
        self.use_portable_mode = false;
        println!("📁 Switched to custom path: {:?}", self.config_path);
        Ok(())
    }
    
    pub fn get_config_mode_description(&self) -> String {
        if self.use_portable_mode {
            "Portable (next to executable)".to_string()
        } else if self.config_path == Self::get_system_config_path() {
            "System (user config directory)".to_string()
        } else {
            "Custom location".to_string()
        }
    }
}
