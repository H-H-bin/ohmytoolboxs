use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::mpsc;
use serde::{Deserialize, Serialize};
use egui::{ComboBox, Grid, RichText, ScrollArea, Ui, ProgressBar};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastbootDevice {
    pub serial: String,
    pub status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum FastbootFunction {
    DeviceInfo,
    FlashOperations,
    BootloaderManagement,
    PartitionOperations,
    SystemOperations,
}

impl FastbootFunction {
    pub fn all() -> Vec<Self> {
        vec![
            Self::DeviceInfo,
            Self::FlashOperations,
            Self::BootloaderManagement,
            Self::PartitionOperations,
            Self::SystemOperations,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "Device Information",
            Self::FlashOperations => "Flash Operations",
            Self::BootloaderManagement => "Bootloader Management",
            Self::PartitionOperations => "Partition Operations",
            Self::SystemOperations => "System Operations",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "📊",
            Self::FlashOperations => "⚡",
            Self::BootloaderManagement => "🔓",
            Self::PartitionOperations => "💾",
            Self::SystemOperations => "⚙️",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "View device information and variables",
            Self::FlashOperations => "Flash firmware images to partitions",
            Self::BootloaderManagement => "Unlock/lock bootloader operations",
            Self::PartitionOperations => "Format and erase partitions",
            Self::SystemOperations => "Boot images and system updates",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FastbootToolsState {
    pub selected_device: Option<String>,
    pub devices: Vec<FastbootDevice>,
    pub last_refresh: String,
    
    // Device Info
    pub device_info: HashMap<String, String>,
    pub device_variables: Vec<String>,
    
    // Flash Operations
    pub selected_partition: String,
    pub image_path: String,
    pub flash_result: String,
    pub flash_progress: f32,
    pub flash_in_progress: bool,
    
    // Bootloader Management
    pub bootloader_unlocked: Option<bool>,
    pub bootloader_result: String,
    
    // Partition Operations  
    pub partition_to_erase: String,
    pub partition_to_format: String,
    pub partition_result: String,
    
    // System Operations
    pub boot_image_path: String,
    pub update_zip_path: String,
    pub system_result: String,
    pub reboot_mode: String,
    
    // Auto-refresh tracking
    #[serde(skip)]
    pub initial_refresh_done: bool,
    
    // Function visibility settings
    pub fastboot_function_visibility: HashMap<FastbootFunction, bool>,
    
    // Tool instance
    #[serde(skip)]
    pub fastboot_tool: FastbootTool,
}

impl Default for FastbootToolsState {
    fn default() -> Self {
        // Initialize all fastboot functions as visible by default
        let mut fastboot_function_visibility = HashMap::new();
        for function in FastbootFunction::all() {
            fastboot_function_visibility.insert(function, true);
        }
        
        Self {
            selected_device: None,
            devices: Vec::new(),
            last_refresh: "Never".to_string(),
            device_info: HashMap::new(),
            device_variables: vec![
                "product".to_string(),
                "variant".to_string(),
                "version-bootloader".to_string(),
                "version-baseband".to_string(),
                "serialno".to_string(),
                "secure".to_string(),
                "unlocked".to_string(),
                "max-download-size".to_string(),
            ],
            selected_partition: "boot".to_string(),
            image_path: String::new(),
            flash_result: String::new(),
            flash_progress: 0.0,
            flash_in_progress: false,
            bootloader_unlocked: None,
            bootloader_result: String::new(),
            partition_to_erase: "cache".to_string(),
            partition_to_format: "userdata".to_string(),
            partition_result: String::new(),
            boot_image_path: String::new(),
            update_zip_path: String::new(),
            system_result: String::new(),
            reboot_mode: "system".to_string(),
            initial_refresh_done: false,
            fastboot_function_visibility,
            fastboot_tool: FastbootTool::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FastbootOperation {
    Flash { partition: String, image_path: String },
    Erase { partition: String },
    Reboot { mode: Option<String> },
    GetVar { variable: String },
    Unlock,
    Lock,
    Format { partition: String },
    Boot { image_path: String },
    FlashAll { zip_path: String },
}

#[derive(Debug, Clone)]
pub struct FastbootResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FastbootTool {
    fastboot_path: String,
}

impl Default for FastbootTool {
    fn default() -> Self {
        Self::new()
    }
}

impl FastbootTool {
    pub fn new() -> Self {
        Self {
            fastboot_path: "fastboot".to_string(), // Assumes fastboot is in PATH
        }
    }

    pub fn with_custom_path(path: String) -> Self {
        Self {
            fastboot_path: path,
        }
    }

    /// Check if fastboot is available
    pub fn is_available(&self) -> bool {
        Command::new(&self.fastboot_path)
            .args(&["--version"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// List all connected fastboot devices
    pub fn list_devices(&self) -> Result<Vec<FastbootDevice>, Box<dyn std::error::Error>> {
        let output = Command::new(&self.fastboot_path)
            .args(&["devices"])
            .output()?;

        if !output.status.success() {
            return Err(format!("Fastboot command failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in output_str.lines() {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    devices.push(FastbootDevice {
                        serial: parts[0].to_string(),
                        status: parts[1].to_string(),
                    });
                }
            }
        }

        Ok(devices)
    }

    /// Execute a fastboot operation
    pub fn execute_operation(
        &self,
        operation: FastbootOperation,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(&self.fastboot_path);

        // Add device serial if specified
        if let Some(serial) = device_serial {
            cmd.args(&["-s", serial]);
        }

        // Add operation-specific arguments
        match operation {
            FastbootOperation::Flash { partition, image_path } => {
                cmd.args(&["flash", &partition, &image_path]);
            }
            FastbootOperation::Erase { partition } => {
                cmd.args(&["erase", &partition]);
            }
            FastbootOperation::Reboot { mode } => {
                if let Some(mode) = mode {
                    cmd.args(&["reboot", &mode]);
                } else {
                    cmd.args(&["reboot"]);
                }
            }
            FastbootOperation::GetVar { variable } => {
                cmd.args(&["getvar", &variable]);
            }
            FastbootOperation::Unlock => {
                cmd.args(&["flashing", "unlock"]);
            }
            FastbootOperation::Lock => {
                cmd.args(&["flashing", "lock"]);
            }
            FastbootOperation::Format { partition } => {
                cmd.args(&["format", &partition]);
            }
            FastbootOperation::Boot { image_path } => {
                cmd.args(&["boot", &image_path]);
            }
            FastbootOperation::FlashAll { zip_path } => {
                cmd.args(&["update", &zip_path]);
            }
        }

        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(FastbootResult {
            success: output.status.success(),
            output: stdout.to_string(),
            error: if stderr.is_empty() { None } else { Some(stderr.to_string()) },
        })
    }

    /// Execute fastboot operation with real-time output
    pub fn execute_with_output<F>(
        &self,
        operation: FastbootOperation,
        device_serial: Option<&str>,
        mut callback: F,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>>
    where
        F: FnMut(String) + Send + 'static,
    {
        let mut cmd = Command::new(&self.fastboot_path);

        // Add device serial if specified
        if let Some(serial) = device_serial {
            cmd.args(&["-s", serial]);
        }

        // Add operation-specific arguments
        match operation {
            FastbootOperation::Flash { partition, image_path } => {
                cmd.args(&["flash", &partition, &image_path]);
            }
            FastbootOperation::Erase { partition } => {
                cmd.args(&["erase", &partition]);
            }
            FastbootOperation::Reboot { mode } => {
                if let Some(mode) = mode {
                    cmd.args(&["reboot", &mode]);
                } else {
                    cmd.args(&["reboot"]);
                }
            }
            FastbootOperation::GetVar { variable } => {
                cmd.args(&["getvar", &variable]);
            }
            FastbootOperation::Unlock => {
                cmd.args(&["flashing", "unlock"]);
            }
            FastbootOperation::Lock => {
                cmd.args(&["flashing", "lock"]);
            }
            FastbootOperation::Format { partition } => {
                cmd.args(&["format", &partition]);
            }
            FastbootOperation::Boot { image_path } => {
                cmd.args(&["boot", &image_path]);
            }
            FastbootOperation::FlashAll { zip_path } => {
                cmd.args(&["update", &zip_path]);
            }
        }

        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let (tx, rx) = mpsc::channel();

        // Handle stdout
        if let Some(stdout) = child.stdout.take() {
            let tx_stdout = tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = tx_stdout.send(line);
                    }
                }
            });
        }

        // Handle stderr
        if let Some(stderr) = child.stderr.take() {
            let tx_stderr = tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = tx_stderr.send(line);
                    }
                }
            });
        }

        drop(tx);

        let mut output_lines = Vec::new();
        let mut error_lines = Vec::new();

        // Collect output
        while let Ok(line) = rx.recv() {
            callback(line.clone());
            if line.contains("FAILED") || line.contains("error") {
                error_lines.push(line);
            } else {
                output_lines.push(line);
            }
        }

        let status = child.wait()?;

        Ok(FastbootResult {
            success: status.success(),
            output: output_lines.join("\n"),
            error: if error_lines.is_empty() {
                None
            } else {
                Some(error_lines.join("\n"))
            },
        })
    }

    /// Get device variable
    pub fn get_var(
        &self,
        variable: &str,
        device_serial: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = self.execute_operation(
            FastbootOperation::GetVar {
                variable: variable.to_string(),
            },
            device_serial,
        )?;

        if result.success {
            // Parse the output to extract the variable value
            for line in result.output.lines() {
                if line.contains(&format!("{}:", variable)) {
                    if let Some(value) = line.split(':').nth(1) {
                        return Ok(value.trim().to_string());
                    }
                }
            }
            Ok("unknown".to_string())
        } else {
            Err(format!("Failed to get variable {}: {}", variable, 
                result.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    /// Get device info
    pub fn get_device_info(
        &self,
        device_serial: Option<&str>,
    ) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut info = std::collections::HashMap::new();
        
        let variables = vec![
            "product",
            "variant",
            "version-bootloader",
            "version-baseband",
            "serialno",
            "secure",
            "unlocked",
            "off-mode-charge",
            "charger-screen-enabled",
            "battery-soc-ok",
            "battery-voltage",
            "hw-revision",
            "max-download-size",
        ];

        for var in variables {
            if let Ok(value) = self.get_var(var, device_serial) {
                info.insert(var.to_string(), value);
            }
        }

        Ok(info)
    }

    /// Flash a single partition
    pub fn flash_partition(
        &self,
        partition: &str,
        image_path: &str,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(
            FastbootOperation::Flash {
                partition: partition.to_string(),
                image_path: image_path.to_string(),
            },
            device_serial,
        )
    }

    /// Erase a partition
    pub fn erase_partition(
        &self,
        partition: &str,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(
            FastbootOperation::Erase {
                partition: partition.to_string(),
            },
            device_serial,
        )
    }

    /// Reboot device
    pub fn reboot(
        &self,
        mode: Option<&str>,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(
            FastbootOperation::Reboot {
                mode: mode.map(|s| s.to_string()),
            },
            device_serial,
        )
    }

    /// Unlock bootloader
    pub fn unlock_bootloader(
        &self,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(FastbootOperation::Unlock, device_serial)
    }

    /// Lock bootloader
    pub fn lock_bootloader(
        &self,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(FastbootOperation::Lock, device_serial)
    }

    /// Boot from image without flashing
    pub fn boot_image(
        &self,
        image_path: &str,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(
            FastbootOperation::Boot {
                image_path: image_path.to_string(),
            },
            device_serial,
        )
    }

    /// Flash all partitions from update zip
    pub fn flash_all(
        &self,
        zip_path: &str,
        device_serial: Option<&str>,
    ) -> Result<FastbootResult, Box<dyn std::error::Error>> {
        self.execute_operation(
            FastbootOperation::FlashAll {
                zip_path: zip_path.to_string(),
            },
            device_serial,
        )
    }
}

pub fn show_fastboot_tools(ui: &mut egui::Ui, state: &mut FastbootToolsState) {
    ui.heading("⚡ Android Fastboot Tools");
    ui.separator();
    
    // Auto-refresh devices on first load
    if !state.initial_refresh_done {
        refresh_fastboot_devices(state);
        state.initial_refresh_done = true;
    }
    
    // Device Selection Section
    ui.group(|ui| {
        ui.label(RichText::new("Device Management").strong());
        
        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh Devices").clicked() {
                refresh_fastboot_devices(state);
            }
            
            ui.label(format!("Last refresh: {}", state.last_refresh));
        });
        
        ui.horizontal(|ui| {
            ui.label("Selected Device:");
            ComboBox::from_label("")
                .selected_text(
                    state.selected_device.as_deref().unwrap_or("No device selected")
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.selected_device, None, "None");
                    for device in &state.devices {
                        let display_text = format!("{} ({})", device.serial, device.status);
                        ui.selectable_value(
                            &mut state.selected_device,
                            Some(device.serial.clone()),
                            display_text,
                        );
                    }
                });
        });
        
        // Show connection status
        if state.devices.len() == 1 && state.selected_device.is_some() {
            ui.horizontal(|ui| {
                ui.label("🔗");
                ui.label(RichText::new("Auto-connected to single device").color(egui::Color32::from_rgb(0, 200, 0)));
            });
        } else if state.devices.len() > 1 {
            ui.horizontal(|ui| {
                ui.label("📱");
                ui.label(RichText::new(format!("{} devices available", state.devices.len())).weak());
            });
        } else if state.devices.is_empty() {
            ui.horizontal(|ui| {
                ui.label("⚠");
                ui.label(RichText::new("No devices found").color(egui::Color32::from_rgb(200, 200, 0)));
                ui.label("- Make sure device is in fastboot mode");
            });
        }
        
        if !state.devices.is_empty() {
            ui.collapsing("📱 Connected Devices", |ui| {
                for device in &state.devices {
                    ui.horizontal(|ui| {
                        ui.label("📱");
                        ui.label(&device.serial);
                        ui.label(format!("({})", device.status));
                    });
                }
            });
        }
    });
    
    ui.separator();
    
    if state.selected_device.is_none() {
        ui.colored_label(egui::Color32::YELLOW, "⚠️ Please select a device to use Fastboot tools");
        return;
    }

    // Wrap all Fastboot functionalities in a scroll area
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            // Different Fastboot functionalities in collapsing sections
            if state.fastboot_function_visibility.get(&FastbootFunction::DeviceInfo).copied().unwrap_or(true) {
                ui.collapsing("📊 Device Information", |ui| show_device_info_tab(ui, state));
            }
            if state.fastboot_function_visibility.get(&FastbootFunction::FlashOperations).copied().unwrap_or(true) {
                ui.collapsing("⚡ Flash Operations", |ui| show_flash_operations_tab(ui, state));
            }
            if state.fastboot_function_visibility.get(&FastbootFunction::BootloaderManagement).copied().unwrap_or(true) {
                ui.collapsing("🔓 Bootloader Management", |ui| show_bootloader_management_tab(ui, state));
            }
            if state.fastboot_function_visibility.get(&FastbootFunction::PartitionOperations).copied().unwrap_or(true) {
                ui.collapsing("💾 Partition Operations", |ui| show_partition_operations_tab(ui, state));
            }
            if state.fastboot_function_visibility.get(&FastbootFunction::SystemOperations).copied().unwrap_or(true) {
                ui.collapsing("⚙️ System Operations", |ui| show_system_operations_tab(ui, state));
            }
        });
}

fn show_device_info_tab(ui: &mut Ui, state: &mut FastbootToolsState) {
    ui.horizontal(|ui| {
        if ui.button("📊 Get Device Info").clicked() {
            get_device_info(state);
        }
        
        if ui.button("🔍 Check Fastboot").clicked() {
            check_fastboot_availability(state);
        }
    });
    
    ui.separator();
    
    if !state.device_info.is_empty() {
        ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            Grid::new("device_info_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    for (key, value) in &state.device_info {
                        ui.label(RichText::new(key).strong());
                        ui.label(value);
                        ui.end_row();
                    }
                });
        });
    }
}

fn show_flash_operations_tab(ui: &mut Ui, state: &mut FastbootToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Flash Image to Partition").strong());
        
        Grid::new("flash_grid").num_columns(2).show(ui, |ui| {
            ui.label("Partition:");
            ComboBox::from_label("")
                .selected_text(&state.selected_partition)
                .show_ui(ui, |ui| {
                    for partition in &[
                        partitions::BOOT,
                        partitions::RECOVERY,
                        partitions::SYSTEM,
                        partitions::VENDOR,
                        partitions::BOOTLOADER,
                        partitions::RADIO,
                    ] {
                        ui.selectable_value(&mut state.selected_partition, partition.to_string(), *partition);
                    }
                });
            ui.end_row();
            
            ui.label("Image Path:");
            ui.text_edit_singleline(&mut state.image_path);
            ui.end_row();
        });
          ui.horizontal(|ui| {
            let flash_button_enabled = !state.image_path.is_empty() && !state.flash_in_progress;
            if ui.add_enabled(flash_button_enabled, egui::Button::new("⚡ Flash Image")).clicked() {
                flash_image(state);
            }
            
            if ui.button("🗂️ Browse").clicked() {
                // TODO: Implement file browser
                state.flash_result = "File browser not implemented yet".to_string();
            }
        });
        
        if state.flash_in_progress {
            ui.horizontal(|ui| {
                ui.label("Flashing:");
                ui.add(ProgressBar::new(state.flash_progress).show_percentage());
            });
        }
    });
    
    if !state.flash_result.is_empty() {
        ui.separator();
        ui.label("Flash Result:");
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.code(&state.flash_result);
        });
    }
}

fn show_bootloader_management_tab(ui: &mut Ui, state: &mut FastbootToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Bootloader Lock/Unlock").strong());
        
        ui.horizontal(|ui| {
            if ui.button("🔓 Unlock Bootloader").clicked() {
                unlock_bootloader(state);
            }
            
            if ui.button("🔒 Lock Bootloader").clicked() {
                lock_bootloader(state);
            }
            
            if ui.button("🔍 Check Lock Status").clicked() {
                check_bootloader_status(state);
            }
        });
        
        if let Some(unlocked) = state.bootloader_unlocked {
            ui.horizontal(|ui| {
                ui.label("Bootloader Status:");
                if unlocked {
                    ui.label(RichText::new("🔓 Unlocked").color(egui::Color32::GREEN));
                } else {
                    ui.label(RichText::new("🔒 Locked").color(egui::Color32::RED));
                }
            });
        }
        
        ui.small("⚠️ Warning: Unlocking bootloader will erase all user data!");
    });
    
    if !state.bootloader_result.is_empty() {
        ui.separator();
        ui.label("Result:");
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.code(&state.bootloader_result);
        });
    }
}

fn show_partition_operations_tab(ui: &mut Ui, state: &mut FastbootToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Partition Management").strong());
        
        Grid::new("partition_grid").num_columns(2).show(ui, |ui| {
            ui.label("Erase Partition:");
            ComboBox::from_label("erase")
                .selected_text(&state.partition_to_erase)
                .show_ui(ui, |ui| {
                    for partition in &[
                        partitions::CACHE,
                        partitions::USERDATA,
                        partitions::BOOT,
                        partitions::RECOVERY,
                        partitions::SYSTEM,
                    ] {
                        ui.selectable_value(&mut state.partition_to_erase, partition.to_string(), *partition);
                    }
                });
            ui.end_row();
            
            ui.label("Format Partition:");
            ComboBox::from_label("format")
                .selected_text(&state.partition_to_format)
                .show_ui(ui, |ui| {
                    for partition in &[
                        partitions::USERDATA,
                        partitions::CACHE,
                        partitions::SYSTEM,
                    ] {
                        ui.selectable_value(&mut state.partition_to_format, partition.to_string(), *partition);
                    }
                });
            ui.end_row();
        });
        
        ui.horizontal(|ui| {
            if ui.button("🗑️ Erase Partition").clicked() {
                erase_partition(state);
            }
            
            if ui.button("💾 Format Partition").clicked() {
                format_partition(state);
            }
        });
        
        ui.small("⚠️ Warning: These operations will permanently delete data!");
    });
    
    if !state.partition_result.is_empty() {
        ui.separator();
        ui.label("Result:");
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.code(&state.partition_result);
        });
    }
}

fn show_system_operations_tab(ui: &mut Ui, state: &mut FastbootToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Boot and System Operations").strong());
        
        Grid::new("system_grid").num_columns(2).show(ui, |ui| {
            ui.label("Boot Image:");
            ui.text_edit_singleline(&mut state.boot_image_path);
            ui.end_row();
            
            ui.label("Update ZIP:");
            ui.text_edit_singleline(&mut state.update_zip_path);
            ui.end_row();
            
            ui.label("Reboot Mode:");
            ComboBox::from_label("reboot")
                .selected_text(&state.reboot_mode)
                .show_ui(ui, |ui| {
                    for mode in &["system", "bootloader", "recovery", "fastboot"] {
                        ui.selectable_value(&mut state.reboot_mode, mode.to_string(), *mode);
                    }
                });
            ui.end_row();
        });
          ui.horizontal(|ui| {
            let boot_enabled = !state.boot_image_path.is_empty();
            if ui.add_enabled(boot_enabled, egui::Button::new("🚀 Boot Image")).clicked() {
                boot_image_operation(state);
            }
            
            let flash_all_enabled = !state.update_zip_path.is_empty();
            if ui.add_enabled(flash_all_enabled, egui::Button::new("📦 Flash All")).clicked() {
                flash_all_operation(state);
            }
            
            if ui.button("🔄 Reboot").clicked() {
                reboot_device(state);
            }
        });
    });
    
    if !state.system_result.is_empty() {
        ui.separator();
        ui.label("Result:");
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.code(&state.system_result);
        });
    }
}

// Implementation functions
fn refresh_fastboot_devices(state: &mut FastbootToolsState) {
    match state.fastboot_tool.list_devices() {
        Ok(devices) => {
            state.devices = devices;
            state.last_refresh = chrono::Utc::now().format("%H:%M:%S").to_string();
            
            // Auto-connect to device if there's only one device available
            if state.devices.len() == 1 {
                let device_serial = state.devices[0].serial.clone();
                state.selected_device = Some(device_serial);
            }
        }
        Err(e) => {
            state.devices.clear();
            state.last_refresh = format!("Error: {}", e);
        }
    }
}

fn get_device_info(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        state.device_info.clear();
        
        match state.fastboot_tool.get_device_info(Some(device_serial)) {
            Ok(info) => {
                state.device_info = info;
            }
            Err(e) => {
                state.device_info.insert("Error".to_string(), format!("Failed to get device info: {}", e));
            }
        }
    }
}

fn check_fastboot_availability(state: &mut FastbootToolsState) {
    if state.fastboot_tool.is_available() {
        state.device_info.insert("Fastboot Status".to_string(), "✅ Available".to_string());
    } else {
        state.device_info.insert("Fastboot Status".to_string(), "❌ Not found in PATH".to_string());
    }
}

fn unlock_bootloader(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.unlock_bootloader(Some(device_serial)) {
            Ok(result) => {
                state.bootloader_result = format!("Unlock result: {}", result.output);
                state.bootloader_unlocked = Some(result.success);
            }
            Err(e) => {
                state.bootloader_result = format!("Unlock failed: {}", e);
            }
        }
    }
}

fn lock_bootloader(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.lock_bootloader(Some(device_serial)) {
            Ok(result) => {
                state.bootloader_result = format!("Lock result: {}", result.output);
                state.bootloader_unlocked = Some(!result.success);
            }
            Err(e) => {
                state.bootloader_result = format!("Lock failed: {}", e);
            }
        }
    }
}

fn check_bootloader_status(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.get_var("unlocked", Some(device_serial)) {
            Ok(value) => {
                state.bootloader_unlocked = Some(value == "yes");
                state.bootloader_result = format!("Bootloader unlock status: {}", value);
            }
            Err(e) => {
                state.bootloader_result = format!("Failed to check status: {}", e);
            }
        }
    }
}

fn erase_partition(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.erase_partition(&state.partition_to_erase, Some(device_serial)) {
            Ok(result) => {
                state.partition_result = format!("Erase result: {}", result.output);
            }
            Err(e) => {
                state.partition_result = format!("Erase failed: {}", e);
            }
        }
    }
}

fn format_partition(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.execute_operation(
            FastbootOperation::Format {
                partition: state.partition_to_format.clone(),
            },
            Some(device_serial),
        ) {
            Ok(result) => {
                state.partition_result = format!("Format result: {}", result.output);
            }
            Err(e) => {
                state.partition_result = format!("Format failed: {}", e);
            }
        }
    }
}

fn reboot_device(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        let mode = if state.reboot_mode == "system" { None } else { Some(state.reboot_mode.as_str()) };
        
        match state.fastboot_tool.reboot(mode, Some(device_serial)) {
            Ok(result) => {
                state.system_result = format!("Reboot result: {}", result.output);
            }
            Err(e) => {
                state.system_result = format!("Reboot failed: {}", e);
            }
        }
    }
}

fn flash_image(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        state.flash_in_progress = true;
        state.flash_progress = 0.0;
        
        match state.fastboot_tool.flash_partition(&state.selected_partition, &state.image_path, Some(device_serial)) {
            Ok(result) => {
                state.flash_result = format!("Flash result: {}", result.output);
                state.flash_progress = 1.0;
            }
            Err(e) => {
                state.flash_result = format!("Flash failed: {}", e);
                state.flash_progress = 0.0;
            }
        }
        
        state.flash_in_progress = false;
    }
}

fn boot_image_operation(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.boot_image(&state.boot_image_path, Some(device_serial)) {
            Ok(result) => {
                state.system_result = format!("Boot result: {}", result.output);
            }
            Err(e) => {
                state.system_result = format!("Boot failed: {}", e);
            }
        }
    }
}

fn flash_all_operation(state: &mut FastbootToolsState) {
    if let Some(device_serial) = &state.selected_device {
        match state.fastboot_tool.flash_all(&state.update_zip_path, Some(device_serial)) {
            Ok(result) => {
                state.system_result = format!("Flash all result: {}", result.output);
            }
            Err(e) => {
                state.system_result = format!("Flash all failed: {}", e);
            }
        }
    }
}

// Common partition names for convenience
pub mod partitions {
    pub const BOOT: &str = "boot";
    pub const RECOVERY: &str = "recovery";
    pub const SYSTEM: &str = "system";
    pub const USERDATA: &str = "userdata";
    pub const CACHE: &str = "cache";
    pub const BOOTLOADER: &str = "bootloader";
    pub const RADIO: &str = "radio";
    pub const VENDOR: &str = "vendor";
    pub const PRODUCT: &str = "product";
    pub const SYSTEM_EXT: &str = "system_ext";
    pub const ODM: &str = "odm";
}

// Common reboot modes
pub mod reboot_modes {
    pub const BOOTLOADER: &str = "bootloader";
    pub const RECOVERY: &str = "recovery";
    pub const FASTBOOT: &str = "fastboot";
}
