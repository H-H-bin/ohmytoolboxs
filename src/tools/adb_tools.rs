use egui::{ComboBox, Grid, RichText, ScrollArea, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: f64, // seconds since monitoring started
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesData {
    pub cpu_usage: VecDeque<DataPoint>,
    pub memory_usage: VecDeque<DataPoint>,
    pub battery_level: VecDeque<DataPoint>,
    pub battery_temperature: VecDeque<DataPoint>,
    #[serde(skip)]
    pub start_time: Option<Instant>,
    pub max_points: usize,
}

impl Default for TimeSeriesData {
    fn default() -> Self {
        Self {
            cpu_usage: VecDeque::new(),
            memory_usage: VecDeque::new(),
            battery_level: VecDeque::new(),
            battery_temperature: VecDeque::new(),
            start_time: None,
            max_points: 1000, // Keep last 1000 data points by default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: String,
    pub name: String,
    pub cpu_percent: String,
    pub memory_kb: String,
    pub user: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdbDevice {
    pub id: String,
    pub status: String,
    pub model: String,
    pub product: String,
    pub transport_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdbToolsState {
    pub selected_device: Option<String>,
    pub devices: Vec<AdbDevice>,
    pub last_refresh: String,
    
    // Device Info
    pub device_info: HashMap<String, String>,
    
    // App Management
    pub package_filter: String,
    pub installed_packages: Vec<String>,
    pub apk_path: String,
    
    // File Operations
    pub local_path: String,
    pub remote_path: String,
    pub file_operation_result: String,
    
    // Shell Commands
    pub shell_command: String,
    pub shell_output: String,
    
    // Logcat
    pub logcat_filter: String,
    pub logcat_output: String,
    pub logcat_running: bool,
    
    // Screen Capture
    pub screenshot_path: String,
    pub screen_record_path: String,
    
    // Port Forwarding
    pub local_port: String,
    pub remote_port: String,
    pub forwarded_ports: Vec<(String, String)>,
    
    // Device Monitoring
    pub monitoring_enabled: bool,
    pub cpu_usage: String,
    pub memory_info: HashMap<String, String>,
    pub process_list: Vec<ProcessInfo>,
    pub selected_process: Option<String>,
    pub process_filter: String,
    pub last_monitor_update: String,
    pub monitor_interval: f32, // seconds
    pub battery_info: HashMap<String, String>,
    pub thermal_info: String,
    pub network_stats: HashMap<String, String>,
    
    #[serde(skip)]
    pub last_update_time: Option<Instant>,
    
    // Time Series Data for Plots
    #[serde(skip)]
    pub time_series: TimeSeriesData,
    pub show_plots: bool,
    
    // Auto-refresh tracking
    #[serde(skip)]
    pub initial_refresh_done: bool,
    
    // ADB Function visibility settings
    pub adb_function_visibility: HashMap<AdbFunction, bool>,
    
    // SELinux Management
    pub selinux_output: String,
    pub selinux_file_path: String,
    pub selinux_new_context: String,
    pub selinux_process_query: String,
    
    // Systemd Management
    pub systemd_output: String,
    pub systemd_service_name: String,
    pub systemd_unit_filter: String,
    pub systemd_service_list: Vec<String>,
}

impl Default for AdbToolsState {
    fn default() -> Self {
        // Initialize all ADB functions as visible by default
        let mut adb_function_visibility = HashMap::new();
        for function in AdbFunction::all() {
            adb_function_visibility.insert(function, true);
        }
        
        Self {
            selected_device: None,
            devices: Vec::new(),
            last_refresh: "Never".to_string(),
            device_info: HashMap::new(),
            package_filter: String::new(),
            installed_packages: Vec::new(),
            apk_path: String::new(),
            local_path: String::new(),
            remote_path: "/sdcard/".to_string(),
            file_operation_result: String::new(),
            shell_command: String::new(),
            shell_output: String::new(),
            logcat_filter: String::new(),
            logcat_output: String::new(),
            logcat_running: false,
            screenshot_path: "screenshot.png".to_string(),
            screen_record_path: "screen_record.mp4".to_string(),
            local_port: "8080".to_string(),
            remote_port: "8080".to_string(),
            forwarded_ports: Vec::new(),
            monitoring_enabled: false,
            cpu_usage: String::new(),
            memory_info: HashMap::new(),
            process_list: Vec::new(),
            selected_process: None,
            process_filter: String::new(),
            last_monitor_update: "Never".to_string(),
            monitor_interval: 0.5,
            battery_info: HashMap::new(),
            thermal_info: String::new(),
            network_stats: HashMap::new(),
            last_update_time: None,
            time_series: TimeSeriesData::default(),
            show_plots: false,
            initial_refresh_done: false,
            adb_function_visibility,
            selinux_output: String::new(),
            selinux_file_path: String::new(),
            selinux_new_context: String::new(),
            selinux_process_query: String::new(),
            systemd_output: String::new(),
            systemd_service_name: String::new(),
            systemd_unit_filter: String::new(),
            systemd_service_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdbFunction {
    DeviceInfo,
    DeviceMonitor,
    AppManagement,
    FileOperations,
    ShellCommands,
    Logcat,
    ScreenCapture,
    PortForwarding,
    SelinuxManagement,
    SystemdManagement,
}

impl AdbFunction {
    pub fn all() -> Vec<Self> {
        vec![
            Self::DeviceInfo,
            Self::DeviceMonitor,
            Self::AppManagement,
            Self::FileOperations,
            Self::ShellCommands,
            Self::Logcat,
            Self::ScreenCapture,
            Self::PortForwarding,
            Self::SelinuxManagement,
            Self::SystemdManagement,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "Device Information",
            Self::DeviceMonitor => "Device Monitor",
            Self::AppManagement => "App Management",
            Self::FileOperations => "File Operations",
            Self::ShellCommands => "Shell Commands",
            Self::Logcat => "Logcat",
            Self::ScreenCapture => "Screen Capture",
            Self::PortForwarding => "Port Forwarding",
            Self::SelinuxManagement => "SELinux Management",
            Self::SystemdManagement => "Systemd Management",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "📊",
            Self::DeviceMonitor => "📈",
            Self::AppManagement => "📦",
            Self::FileOperations => "📁",
            Self::ShellCommands => "🖥️",
            Self::Logcat => "📜",
            Self::ScreenCapture => "📱",
            Self::PortForwarding => "🔗",
            Self::SelinuxManagement => "🔒",
            Self::SystemdManagement => "⚙️",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::DeviceInfo => "View device properties, specs, and system information",
            Self::DeviceMonitor => "Real-time monitoring of CPU, memory, battery, and processes",
            Self::AppManagement => "Install, uninstall, and manage Android applications",
            Self::FileOperations => "Transfer files between computer and Android device",
            Self::ShellCommands => "Execute shell commands on the Android device",
            Self::Logcat => "View and filter Android system logs in real-time",
            Self::ScreenCapture => "Take screenshots and record screen activity",
            Self::PortForwarding => "Set up network port forwarding between device and computer",
            Self::SelinuxManagement => "Manage SELinux policies, contexts, and security settings",
            Self::SystemdManagement => "Manage systemd services, units, and system daemon control",
        }
    }
}

pub fn show_adb_tools(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.heading("🤖 Android Debug Bridge (ADB) Tools");
    ui.separator();
    
    // Auto-refresh devices on first load
    if !state.initial_refresh_done {
        refresh_devices(state);
        state.initial_refresh_done = true;
    }
    
    // Real-time monitoring update check
    if state.monitoring_enabled {
        let should_update = match state.last_update_time {
            Some(last_time) => last_time.elapsed() >= Duration::from_secs_f32(state.monitor_interval),
            None => true,
        };
        
        if should_update {
            update_monitoring_data(state);
            state.last_update_time = Some(Instant::now());
            
            // Initialize time series start time if not set
            if state.time_series.start_time.is_none() {
                state.time_series.start_time = Some(Instant::now());
            }
            
            ui.ctx().request_repaint(); // Request UI repaint for real-time updates
        }
    }
    
    // Device Selection Section
    ui.group(|ui| {
        ui.label(RichText::new("Device Management").strong());
        
        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh Devices").clicked() {
                refresh_devices(state);
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
                        let display_text = format!("{} ({})", device.id, device.status);
                        ui.selectable_value(
                            &mut state.selected_device,
                            Some(device.id.clone()),
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
            });
        }
        
        if !state.devices.is_empty() {
            ui.collapsing("📱 Connected Devices", |ui| {
                for device in &state.devices {
                    ui.horizontal(|ui| {
                        ui.label("📱");
                        ui.label(&device.id);
                        ui.label(format!("({})", device.status));
                        if !device.model.is_empty() {
                            ui.label(format!("- {}", device.model));
                        }
                    });
                }
            });
        }
    });
    
    ui.separator();
      if state.selected_device.is_none() {
        ui.colored_label(egui::Color32::YELLOW, "⚠️ Please select a device to use ADB tools");
        return;
    }

    // Wrap all ADB functionalities in a scroll area
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            // Different ADB functionalities in collapsing sections
            if state.adb_function_visibility.get(&AdbFunction::DeviceInfo).copied().unwrap_or(true) {
                ui.collapsing("📊 Device Information", |ui| show_device_info_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::DeviceMonitor).copied().unwrap_or(true) {
                ui.collapsing("📈 Device Monitor", |ui| show_device_monitor_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::AppManagement).copied().unwrap_or(true) {
                ui.collapsing("📦 App Management", |ui| show_app_management_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::FileOperations).copied().unwrap_or(true) {
                ui.collapsing("📁 File Operations", |ui| show_file_operations_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::ShellCommands).copied().unwrap_or(true) {
                ui.collapsing("🖥️ Shell Commands", |ui| show_shell_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::Logcat).copied().unwrap_or(true) {
                ui.collapsing("📜 Logcat", |ui| show_logcat_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::ScreenCapture).copied().unwrap_or(true) {
                ui.collapsing("📱 Screen Capture", |ui| show_screen_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::PortForwarding).copied().unwrap_or(true) {
                ui.collapsing("🔗 Port Forwarding", |ui| show_port_forward_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::SelinuxManagement).copied().unwrap_or(true) {
                ui.collapsing("🔒 SELinux Management", |ui| show_selinux_tab(ui, state));
            }
            if state.adb_function_visibility.get(&AdbFunction::SystemdManagement).copied().unwrap_or(true) {
                ui.collapsing("⚙️ Systemd Management", |ui| show_systemd_tab(ui, state));
            }
        });
}

fn show_device_info_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.horizontal(|ui| {
        if ui.button("📊 Get Device Info").clicked() {
            get_device_info(state);
        }
        
        if ui.button("🔋 Battery Info").clicked() {
            get_battery_info(state);
        }
        
        if ui.button("📱 Display Info").clicked() {
            get_display_info(state);
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

fn show_app_management_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Package Management").strong());
        
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut state.package_filter);
            if ui.button("📦 List Packages").clicked() {
                list_packages(state);
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("APK Path:");
            ui.text_edit_singleline(&mut state.apk_path);
            if ui.button("📥 Install APK").clicked() {
                install_apk(state);
            }
        });
    });
    
    ui.separator();
    
    if !state.installed_packages.is_empty() {
        ui.label(format!("Found {} packages:", state.installed_packages.len()));
        ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            let mut package_to_remove: Option<String> = None;
            
            for package in &state.installed_packages {
                ui.horizontal(|ui| {
                    ui.label("📦");
                    ui.label(package);
                    if ui.small_button("🗑️").clicked() {
                        package_to_remove = Some(package.clone());
                    }
                });
            }
            
            // Handle uninstall outside the iteration
            if let Some(package_name) = package_to_remove {
                uninstall_package(state, &package_name);
            }
        });
    }
}

fn show_file_operations_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("File Transfer").strong());
        
        Grid::new("file_ops_grid").num_columns(2).show(ui, |ui| {
            ui.label("Local Path:");
            ui.text_edit_singleline(&mut state.local_path);
            ui.end_row();
            
            ui.label("Remote Path:");
            ui.text_edit_singleline(&mut state.remote_path);
            ui.end_row();
        });
        
        ui.horizontal(|ui| {
            if ui.button("📤 Push to Device").clicked() {
                push_file(state);
            }
            if ui.button("📥 Pull from Device").clicked() {
                pull_file(state);
            }
            if ui.button("📁 List Remote Dir").clicked() {
                list_remote_directory(state);
            }
        });
    });
    
    if !state.file_operation_result.is_empty() {
        ui.separator();
        ui.label("Result:");
        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.label(&state.file_operation_result);
        });
    }
}

fn show_shell_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("ADB Shell").strong());
        
        ui.horizontal(|ui| {
            ui.label("Command:");
            ui.text_edit_singleline(&mut state.shell_command);
            if ui.button("▶️ Execute").clicked() {
                execute_shell_command(state);
            }
        });
        
        // Common commands
        ui.horizontal(|ui| {
            if ui.small_button("ls /sdcard/").clicked() {
                state.shell_command = "ls /sdcard/".to_string();
            }
            if ui.small_button("ps").clicked() {
                state.shell_command = "ps".to_string();
            }
            if ui.small_button("df -h").clicked() {
                state.shell_command = "df -h".to_string();
            }
            if ui.small_button("getprop").clicked() {
                state.shell_command = "getprop".to_string();
            }
        });
    });
    
    if !state.shell_output.is_empty() {
        ui.separator();
        ui.label("Output:");
        ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            ui.code(&state.shell_output);
        });
    }
}

fn show_logcat_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Logcat").strong());
        
        ui.horizontal(|ui| {
            ui.label("Filter/Tag:");
            ui.text_edit_singleline(&mut state.logcat_filter);
            
            if !state.logcat_running {
                if ui.button("▶️ Start Logcat").clicked() {
                    start_logcat(state);
                }
            } else {
                if ui.button("⏹️ Stop Logcat").clicked() {
                    stop_logcat(state);
                }
            }
            
            if ui.button("🗑️ Clear").clicked() {
                clear_logcat(state);
            }
        });
    });
    
    ui.separator();
    
    if !state.logcat_output.is_empty() {
        ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            ui.code(&state.logcat_output);
        });
    }
}

fn show_screen_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Screen Capture").strong());
        
        Grid::new("screen_grid").num_columns(2).show(ui, |ui| {
            ui.label("Screenshot Path:");
            ui.text_edit_singleline(&mut state.screenshot_path);
            ui.end_row();
            
            ui.label("Screen Record Path:");
            ui.text_edit_singleline(&mut state.screen_record_path);
            ui.end_row();
        });
        
        ui.horizontal(|ui| {
            if ui.button("📸 Take Screenshot").clicked() {
                take_screenshot(state);
            }
            if ui.button("🎥 Start Recording").clicked() {
                start_screen_record(state);
            }
            if ui.button("⏹️ Stop Recording").clicked() {
                stop_screen_record(state);
            }
        });
    });
}

fn show_port_forward_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.group(|ui| {
        ui.label(RichText::new("Port Forwarding").strong());
        
        Grid::new("port_grid").num_columns(2).show(ui, |ui| {
            ui.label("Local Port:");
            ui.text_edit_singleline(&mut state.local_port);
            ui.end_row();
            
            ui.label("Remote Port:");
            ui.text_edit_singleline(&mut state.remote_port);
            ui.end_row();
        });
        
        ui.horizontal(|ui| {
            if ui.button("➡️ Forward Port").clicked() {
                forward_port(state);
            }
            if ui.button("🗑️ Remove All").clicked() {
                remove_all_forwards(state);
            }
        });
    });
    
    if !state.forwarded_ports.is_empty() {
        ui.separator();
        ui.label("Active Port Forwards:");
        for (local, remote) in &state.forwarded_ports.clone() {
            ui.horizontal(|ui| {
                ui.label(format!("{}:{} → {}", local, remote, remote));
                if ui.small_button("❌").clicked() {
                    remove_port_forward(state, local);
                }
            });
        }
    }
}

fn show_device_monitor_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    // Monitor controls
    ui.group(|ui| {
        ui.label(RichText::new("Real-time Monitoring").strong());
        
        ui.horizontal(|ui| {
            let button_text = if state.monitoring_enabled {
                "⏹️ Stop Monitoring"
            } else {
                "▶️ Start Monitoring"
            };
            
            if ui.button(button_text).clicked() {
                state.monitoring_enabled = !state.monitoring_enabled;
                if state.monitoring_enabled {
                    state.time_series.start_time = Some(Instant::now());
                    update_monitoring_data(state);
                    state.last_update_time = Some(Instant::now());
                } else {
                    state.last_update_time = None;
                }
            }
            
            ui.label("Interval (seconds):");
            ui.add(egui::Slider::new(&mut state.monitor_interval, 1.0..=10.0));
            
            if ui.button("🔄 Update Now").clicked() {
                update_monitoring_data(state);
                state.last_update_time = Some(Instant::now());
            }
            
            ui.checkbox(&mut state.show_plots, "📊 Show Plots");
            
            // Configuration for max data points
            ui.horizontal(|ui| {
                ui.label("Max data points:");
                let mut max_points_f32 = state.time_series.max_points as f32;
                if ui.add(egui::DragValue::new(&mut max_points_f32)
                    .range(10.0..=10000.0)
                    .speed(10.0)
                    .suffix(" points")).changed() {
                    let new_max = max_points_f32 as usize;
                    if new_max != state.time_series.max_points {
                        state.time_series.max_points = new_max;
                        // Trim existing data if new limit is smaller
                        trim_time_series_data(state);
                    }
                }
            });
        });
        
        if !state.last_monitor_update.is_empty() {
            ui.label(format!("Last update: {}", state.last_monitor_update));
        }
    });
    
    ui.separator();
    
    // Plot Section
    if state.show_plots && !state.time_series.cpu_usage.is_empty() {
        ui.group(|ui| {
            ui.label(RichText::new("📈 Performance Trends").strong());
            
            // CPU Usage Plot
            ui.label("CPU Load Average");
            Plot::new("cpu_plot")
                .height(150.0)
                .view_aspect(3.0)
                .show(ui, |plot_ui| {
                    let cpu_points: PlotPoints = state.time_series.cpu_usage
                        .iter()
                        .map(|p| [p.timestamp, p.value])
                        .collect();
                    
                    if !state.time_series.cpu_usage.is_empty() {
                        plot_ui.line(
                            Line::new(cpu_points)
                                .color(egui::Color32::from_rgb(255, 100, 100))
                                .name("CPU Load")
                        );
                    }
                });
            
            // Memory Usage Plot
            ui.label("Memory Usage %");
            Plot::new("memory_plot")
                .height(150.0)
                .view_aspect(3.0)
                .show(ui, |plot_ui| {
                    let memory_points: PlotPoints = state.time_series.memory_usage
                        .iter()
                        .map(|p| [p.timestamp, p.value])
                        .collect();
                    
                    if !state.time_series.memory_usage.is_empty() {
                        plot_ui.line(
                            Line::new(memory_points)
                                .color(egui::Color32::from_rgb(100, 255, 100))
                                .name("Memory Usage")
                        );
                    }
                });
            
            // Battery Level Plot
            if !state.time_series.battery_level.is_empty() {
                ui.label("Battery Level %");
                Plot::new("battery_plot")
                    .height(150.0)
                    .view_aspect(3.0)
                    .show(ui, |plot_ui| {
                        let battery_points: PlotPoints = state.time_series.battery_level
                            .iter()
                            .map(|p| [p.timestamp, p.value])
                            .collect();
                        
                        plot_ui.line(
                            Line::new(battery_points)
                                .color(egui::Color32::from_rgb(100, 100, 255))
                                .name("Battery Level")
                        );
                    });
            }
            
            // Battery Temperature Plot
            if !state.time_series.battery_temperature.is_empty() {
                ui.label("Battery Temperature °C");
                Plot::new("temp_plot")
                    .height(150.0)
                    .view_aspect(3.0)
                    .show(ui, |plot_ui| {
                        let temp_points: PlotPoints = state.time_series.battery_temperature
                            .iter()
                            .map(|p| [p.timestamp, p.value])
                            .collect();
                        
                        plot_ui.line(
                            Line::new(temp_points)
                                .color(egui::Color32::from_rgb(255, 255, 100))
                                .name("Temperature")
                        );
                    });
            }
            
            ui.horizontal(|ui| {
                if ui.button("🗑️ Clear Plot Data").clicked() {
                    clear_plot_data(state);
                }
                
                ui.label(format!("Data points: {} / {}", state.time_series.cpu_usage.len(), state.time_series.max_points));
            });
        });
        
        ui.separator();
    }
    
    // Real-time data display
    if state.monitoring_enabled || !state.cpu_usage.is_empty() {
        // System Performance Section
        ui.group(|ui| {
            ui.label(RichText::new("📊 System Performance").strong());
            
            Grid::new("system_perf_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("CPU Usage:");
                    ui.label(&state.cpu_usage);
                    ui.end_row();
                    
                    if !state.memory_info.is_empty() {
                        for (key, value) in &state.memory_info {
                            ui.label(key);
                            ui.label(value);
                            ui.end_row();
                        }
                    }
                });
        });
        
        ui.separator();
        
        // Battery and Thermal Section
        ui.group(|ui| {
            ui.label(RichText::new("🔋 Battery & Thermal").strong());
            
            Grid::new("battery_thermal_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    for (key, value) in &state.battery_info {
                        ui.label(key);
                        ui.label(value);
                        ui.end_row();
                    }
                    
                    if !state.thermal_info.is_empty() {
                        ui.label("Thermal Status:");
                        ui.label(&state.thermal_info);
                        ui.end_row();
                    }
                });
        });
        
        ui.separator();
        
        // Network Statistics
        if !state.network_stats.is_empty() {
            ui.group(|ui| {
                ui.label(RichText::new("🌐 Network Statistics").strong());
                
                Grid::new("network_stats_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        for (key, value) in &state.network_stats {
                            ui.label(key);
                            ui.label(value);
                            ui.end_row();
                        }
                    });
            });
            
            ui.separator();
        }
        
        // Process Monitor Section
        ui.group(|ui| {
            ui.label(RichText::new("⚙️ Process Monitor").strong());
            
            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.text_edit_singleline(&mut state.process_filter);
                if ui.button("🔄 Refresh Processes").clicked() {
                    update_process_list(state);
                }
                if ui.button("🎯 Kill Process").clicked() {
                    if let Some(pid) = state.selected_process.clone() {
                        kill_process(state, &pid);
                    }
                }
            });
            
            if !state.process_list.is_empty() {
                ui.label(format!("Running Processes ({})", state.process_list.len()));
                
                ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    Grid::new("process_grid")
                        .num_columns(6)
                        .striped(true)
                        .show(ui, |ui| {
                            // Header
                            ui.label(RichText::new("PID").strong());
                            ui.label(RichText::new("Name").strong());
                            ui.label(RichText::new("CPU%").strong());
                            ui.label(RichText::new("Memory").strong());
                            ui.label(RichText::new("User").strong());
                            ui.label(RichText::new("State").strong());
                            ui.end_row();
                            
                            // Process data
                            let filtered_processes: Vec<_> = state.process_list
                                .iter()
                                .filter(|p| {
                                    if state.process_filter.is_empty() {
                                        true
                                    } else {
                                        p.name.to_lowercase().contains(&state.process_filter.to_lowercase()) ||
                                        p.pid.contains(&state.process_filter)
                                    }
                                })
                                .collect();
                            
                            for process in filtered_processes.iter().take(50) { // Limit to 50 for performance
                                let is_selected = state.selected_process.as_ref() == Some(&process.pid);
                                
                                if ui.selectable_label(is_selected, &process.pid).clicked() {
                                    state.selected_process = Some(process.pid.clone());
                                }
                                ui.label(&process.name);
                                ui.label(&process.cpu_percent);
                                ui.label(&process.memory_kb);
                                ui.label(&process.user);
                                ui.label(&process.state);
                                ui.end_row();
                            }
                            
                            if filtered_processes.len() > 50 {
                                ui.label(format!("... and {} more", filtered_processes.len() - 50));
                                ui.label("");
                                ui.label("");
                                ui.label("");
                                ui.label("");
                                ui.label("");
                                ui.end_row();
                            }
                        });
                });
            }
        });
    } else {
        ui.colored_label(egui::Color32::GRAY, "Start monitoring to see real-time device statistics");
    }
}

// ADB Command Implementation Functions
fn refresh_devices(state: &mut AdbToolsState) {
    match execute_adb_command(&["devices", "-l"]) {
        Ok(output) => {
            state.devices.clear();
            for line in output.lines().skip(1) {
                if line.trim().is_empty() || line.contains("List of devices") {
                    continue;
                }
                
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let device = AdbDevice {
                        id: parts[0].to_string(),
                        status: parts[1].to_string(),
                        model: extract_device_property(line, "model:"),
                        product: extract_device_property(line, "product:"),
                        transport_id: extract_device_property(line, "transport_id:"),
                    };
                    state.devices.push(device);
                }
            }
            
            // Auto-connect to device if there's only one device available
            if state.devices.len() == 1 {
                let device_id = state.devices[0].id.clone();
                state.selected_device = Some(device_id);
                log::info!("Auto-connected to single device: {}", state.devices[0].id);
            } else if state.devices.is_empty() {
                // Clear selection if no devices are available
                state.selected_device = None;
            } else {
                // Multiple devices available - check if current selection is still valid
                if let Some(ref selected_id) = state.selected_device {
                    let device_still_exists = state.devices.iter().any(|d| d.id == *selected_id);
                    if !device_still_exists {
                        // Current selection is no longer available, clear it
                        state.selected_device = None;
                        log::info!("Previously selected device is no longer available, cleared selection");
                    }
                }
            }
            
            state.last_refresh = chrono::Utc::now().format("%H:%M:%S").to_string();
        }
        Err(e) => {
            log::error!("Failed to refresh devices: {}", e);
        }
    }
}

fn get_device_info(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        state.device_info.clear();
        
        // Get various device properties
        let properties = [
            ("Model", "ro.product.model"),
            ("Brand", "ro.product.brand"),
            ("Manufacturer", "ro.product.manufacturer"),
            ("Android Version", "ro.build.version.release"),
            ("API Level", "ro.build.version.sdk"),
            ("Build ID", "ro.build.id"),
            ("Serial", "ro.serialno"),
            ("ABI", "ro.product.cpu.abi"),
            ("Fingerprint", "ro.build.fingerprint"),
        ];
        
        for (key, prop) in properties {
            if let Ok(value) = execute_adb_command(&["-s", device_id, "shell", "getprop", prop]) {
                state.device_info.insert(key.to_string(), value.trim().to_string());
            }
        }
    }
}

fn get_battery_info(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "dumpsys", "battery"]) {
            state.device_info.insert("Battery Info".to_string(), output);
        }
    }
}

fn get_display_info(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "wm", "size"]) {
            state.device_info.insert("Display Size".to_string(), output.trim().to_string());
        }
        if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "wm", "density"]) {
            state.device_info.insert("Display Density".to_string(), output.trim().to_string());
        }
    }
}

fn list_packages(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let mut cmd = vec!["-s", device_id, "shell", "pm", "list", "packages"];
        if !state.package_filter.is_empty() {
            cmd.push(&state.package_filter);
        }
        
        if let Ok(output) = execute_adb_command(&cmd) {
            state.installed_packages = output
                .lines()
                .map(|line| line.replace("package:", ""))
                .collect();
        }
    }
}

fn install_apk(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if !state.apk_path.is_empty() {
            match execute_adb_command(&["-s", device_id, "install", &state.apk_path]) {
                Ok(output) => {
                    state.file_operation_result = format!("Install result: {}", output);
                }
                Err(e) => {
                    state.file_operation_result = format!("Install failed: {}", e);
                }
            }
        }
    }
}

fn uninstall_package(state: &mut AdbToolsState, package: &str) {
    if let Some(device_id) = &state.selected_device {
        match execute_adb_command(&["-s", device_id, "uninstall", package]) {
            Ok(output) => {
                state.file_operation_result = format!("Uninstall result: {}", output);
                list_packages(state); // Refresh package list
            }
            Err(e) => {
                state.file_operation_result = format!("Uninstall failed: {}", e);
            }
        }
    }
}

fn push_file(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if !state.local_path.is_empty() && !state.remote_path.is_empty() {
            match execute_adb_command(&["-s", device_id, "push", &state.local_path, &state.remote_path]) {
                Ok(output) => {
                    state.file_operation_result = format!("Push successful: {}", output);
                }
                Err(e) => {
                    state.file_operation_result = format!("Push failed: {}", e);
                }
            }
        }
    }
}

fn pull_file(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if !state.local_path.is_empty() && !state.remote_path.is_empty() {
            match execute_adb_command(&["-s", device_id, "pull", &state.remote_path, &state.local_path]) {
                Ok(output) => {
                    state.file_operation_result = format!("Pull successful: {}", output);
                }
                Err(e) => {
                    state.file_operation_result = format!("Pull failed: {}", e);
                }
            }
        }
    }
}

fn list_remote_directory(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if !state.remote_path.is_empty() {
            match execute_adb_command(&["-s", device_id, "shell", "ls", "-la", &state.remote_path]) {
                Ok(output) => {
                    state.file_operation_result = output;
                }
                Err(e) => {
                    state.file_operation_result = format!("List failed: {}", e);
                }
            }
        }
    }
}

fn execute_shell_command(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if !state.shell_command.is_empty() {
            match execute_adb_command(&["-s", device_id, "shell", &state.shell_command]) {
                Ok(output) => {
                    state.shell_output = output;
                }
                Err(e) => {
                    state.shell_output = format!("Command failed: {}", e);
                }
            }
        }
    }
}

fn start_logcat(state: &mut AdbToolsState) {
    // This is a simplified version - in a real implementation, you'd want to run this in a background thread
    if let Some(device_id) = &state.selected_device {
        let mut cmd = vec!["-s", device_id, "logcat"];
        if !state.logcat_filter.is_empty() {
            cmd.push("-s");
            cmd.push(&state.logcat_filter);
        }
        cmd.push("-d"); // Dump existing logs
        
        if let Ok(output) = execute_adb_command(&cmd) {
            state.logcat_output = output;
            state.logcat_running = true;
        }
    }
}

fn stop_logcat(state: &mut AdbToolsState) {
    state.logcat_running = false;
}

fn clear_logcat(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let _ = execute_adb_command(&["-s", device_id, "logcat", "-c"]);
        state.logcat_output.clear();
    }
}

fn take_screenshot(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let remote_path = "/sdcard/screenshot.png";
        
        // Take screenshot on device
        if execute_adb_command(&["-s", device_id, "shell", "screencap", "-p", remote_path]).is_ok() {
            // Pull to local path
            match execute_adb_command(&["-s", device_id, "pull", remote_path, &state.screenshot_path]) {
                Ok(_) => {
                    state.file_operation_result = format!("Screenshot saved to: {}", state.screenshot_path);
                }
                Err(e) => {
                    state.file_operation_result = format!("Screenshot failed: {}", e);
                }
            }
        }
    }
}

fn start_screen_record(state: &mut AdbToolsState) {
    // This would typically be run in a background thread
    state.file_operation_result = "Screen recording started (not implemented in demo)".to_string();
}

fn stop_screen_record(state: &mut AdbToolsState) {
    state.file_operation_result = "Screen recording stopped (not implemented in demo)".to_string();
}

fn forward_port(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let forward_spec = format!("tcp:{}:tcp:{}", state.local_port, state.remote_port);
        match execute_adb_command(&["-s", device_id, "forward", &forward_spec]) {
            Ok(_) => {
                state.forwarded_ports.push((state.local_port.clone(), state.remote_port.clone()));
                state.file_operation_result = format!("Port forwarding established: {}:{}", state.local_port, state.remote_port);
            }
            Err(e) => {
                state.file_operation_result = format!("Port forward failed: {}", e);
            }
        }
    }
}

fn remove_port_forward(state: &mut AdbToolsState, local_port: &str) {
    if let Some(device_id) = &state.selected_device {
        let forward_spec = format!("tcp:{}", local_port);
        let _ = execute_adb_command(&["-s", device_id, "forward", "--remove", &forward_spec]);
        state.forwarded_ports.retain(|(lp, _)| lp != local_port);
    }
}

fn remove_all_forwards(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let _ = execute_adb_command(&["-s", device_id, "forward", "--remove-all"]);
        state.forwarded_ports.clear();
    }
}

// Device Monitoring Functions
fn update_monitoring_data(state: &mut AdbToolsState) {
    if let Some(device_id) = state.selected_device.clone() {
        // Update CPU usage
        get_cpu_usage(state, &device_id);
        
        // Update memory information
        get_memory_info(state, &device_id);
        
        // Update battery info
        get_battery_monitoring_info(state, &device_id);
        
        // Update thermal information
        get_thermal_info(state, &device_id);
        
        // Update network statistics
        get_network_stats(state, &device_id);
        
        // Update process list
        update_process_list(state);
        
        // Update timestamp
        state.last_monitor_update = chrono::Utc::now().format("%H:%M:%S").to_string();
        
        // Add data points to time series
        add_time_series_data(state);
    }
}

fn get_cpu_usage(state: &mut AdbToolsState, device_id: &str) {
    // Get CPU usage from /proc/stat
    if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "cat", "/proc/loadavg"]) {
        let parts: Vec<&str> = output.trim().split_whitespace().collect();
        if parts.len() >= 3 {
            state.cpu_usage = format!("Load: {} {} {} (1m 5m 15m)",
                parts[0], parts[1], parts[2]);
        }
    } else {
        state.cpu_usage = "CPU usage unavailable".to_string();
    }
    
    // Try to get more detailed CPU info
    if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "cat", "/proc/cpuinfo"]) {
        let cpu_count = output.lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        
        if cpu_count > 0 {
            state.cpu_usage += &format!(" | {} cores", cpu_count);
        }
    }
}

fn get_memory_info(state: &mut AdbToolsState, device_id: &str) {
    state.memory_info.clear();
    
    // Get memory information from /proc/meminfo
    if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "cat", "/proc/meminfo"]) {
        for line in output.lines().take(10) { // Get first 10 lines
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim();
                let value = line[colon_pos + 1..].trim();
                
                // Format important memory values
                match key {
                    "MemTotal" => state.memory_info.insert("Total Memory".to_string(), value.to_string()),
                    "MemFree" => state.memory_info.insert("Free Memory".to_string(), value.to_string()),
                    "MemAvailable" => state.memory_info.insert("Available Memory".to_string(), value.to_string()),
                    "Buffers" => state.memory_info.insert("Buffers".to_string(), value.to_string()),
                    "Cached" => state.memory_info.insert("Cached".to_string(), value.to_string()),
                    "SwapTotal" => state.memory_info.insert("Swap Total".to_string(), value.to_string()),
                    "SwapFree" => state.memory_info.insert("Swap Free".to_string(), value.to_string()),
                    _ => None,
                };
            }
        }
    }
    
    // Calculate memory usage percentage
    if let (Some(total), Some(available)) = (
        state.memory_info.get("Total Memory").and_then(|s| extract_kb_value(s)),
        state.memory_info.get("Available Memory").and_then(|s| extract_kb_value(s))
    ) {
        let used = total - available;
        let usage_percent = (used as f64 / total as f64) * 100.0;
        state.memory_info.insert("Memory Usage".to_string(), format!("{:.1}%", usage_percent));
    }
}

fn get_battery_monitoring_info(state: &mut AdbToolsState, device_id: &str) {
    state.battery_info.clear();
    
    if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "dumpsys", "battery"]) {
        for line in output.lines() {
            if line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    
                    match key {
                        "level" => { state.battery_info.insert("Battery Level".to_string(), format!("{}%", value)); }
                        "temperature" => { 
                            if let Ok(temp) = value.parse::<f32>() {
                                state.battery_info.insert("Temperature".to_string(), format!("{:.1}°C", temp / 10.0));
                            }
                        }
                        "voltage" => { 
                            if let Ok(voltage) = value.parse::<f32>() {
                                state.battery_info.insert("Voltage".to_string(), format!("{:.2}V", voltage / 1000.0));
                            }
                        }
                        "health" => { state.battery_info.insert("Health".to_string(), value.to_string()); }
                        "status" => { state.battery_info.insert("Status".to_string(), value.to_string()); }
                        "AC powered" => { state.battery_info.insert("AC Powered".to_string(), value.to_string()); }
                        "USB powered" => { state.battery_info.insert("USB Powered".to_string(), value.to_string()); }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn get_thermal_info(state: &mut AdbToolsState, device_id: &str) {
    // Try to get thermal information
    if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "cat", "/sys/class/thermal/thermal_zone0/temp"]) {
        if let Ok(temp) = output.trim().parse::<f32>() {
            state.thermal_info = format!("{:.1}°C", temp / 1000.0);
        }
    } else {
        state.thermal_info = "Not available".to_string();
    }
}

fn get_network_stats(state: &mut AdbToolsState, device_id: &str) {
    state.network_stats.clear();
    
    // Get network interface statistics
    if let Ok(output) = execute_adb_command(&["-s", device_id, "shell", "cat", "/proc/net/dev"]) {
        for line in output.lines().skip(2) { // Skip header lines
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                let interface = parts[0].trim_end_matches(':');
                if interface == "wlan0" || interface == "rmnet0" || interface == "eth0" {
                    let rx_bytes = parts[1];
                    let tx_bytes = parts[9];
                    
                    if let (Ok(rx), Ok(tx)) = (rx_bytes.parse::<u64>(), tx_bytes.parse::<u64>()) {
                        state.network_stats.insert(
                            format!("{} RX", interface),
                            format_bytes(rx)
                        );
                        state.network_stats.insert(
                            format!("{} TX", interface),
                            format_bytes(tx)
                        );
                    }
                }
            }
        }
    }
}

fn update_process_list(state: &mut AdbToolsState) {
    if let Some(device_id) = state.selected_device.clone() {
        state.process_list.clear();
        
        // Get process information using ps command
        if let Ok(output) = execute_adb_command(&["-s", &device_id, "shell", "ps", "-o", "user,group,pid,ppid,pgid,etime,nice,rgroup,ruser,time,tty,vsz,sid,stat,rss,comm,args,label"]) {
            for line in output.lines().skip(1) { // Skip header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let process = ProcessInfo {
                        pid: parts[0].to_string(),
                        name: parts[1].to_string(),
                        cpu_percent: parts[2].to_string(),
                        memory_kb: format!("{} KB", parts[3]),
                        user: parts[4].to_string(),
                        state: parts[5].to_string(),
                    };
                    state.process_list.push(process);
                }
            }
        } else {
            // Fallback to simpler ps command
            if let Ok(output) = execute_adb_command(&["-s", &device_id, "shell", "ps"]) {
                for line in output.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 9 {
                        let process = ProcessInfo {
                            pid: parts[1].to_string(),
                            name: parts[8].to_string(),
                            cpu_percent: "N/A".to_string(),
                            memory_kb: format!("{} KB", parts[4]),
                            user: parts[0].to_string(),
                            state: parts[2].to_string(),
                        };
                        state.process_list.push(process);
                    }
                }
            }
        }
        
        // Sort by PID for consistency
        state.process_list.sort_by(|a, b| {
            a.pid.parse::<u32>().unwrap_or(0).cmp(&b.pid.parse::<u32>().unwrap_or(0))
        });
    }
}

fn kill_process(state: &mut AdbToolsState, pid: &str) {
    if let Some(device_id) = state.selected_device.clone() {
        match execute_adb_command(&["-s", &device_id, "shell", "kill", pid]) {
            Ok(_) => {
                state.file_operation_result = format!("Process {} killed successfully", pid);
                // Refresh process list
                update_process_list(state);
            }
            Err(e) => {
                state.file_operation_result = format!("Failed to kill process {}: {}", pid, e);
            }
        }
    }
}

// SELinux Management Tab
fn show_selinux_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    ui.horizontal(|ui| {
        if ui.button("🔒 Get SELinux Status").clicked() {
            get_selinux_status(state);
        }
        
        if ui.button("📋 List Contexts").clicked() {
            get_selinux_contexts(state);
        }
        
        if ui.button("⚙️ Get Policy").clicked() {
            get_selinux_policy(state);
        }
    });
    
    ui.separator();
    
    // SELinux Mode Control
    ui.group(|ui| {
        ui.label(RichText::new("SELinux Mode Control").strong());
        ui.horizontal(|ui| {
            if ui.button("🟢 Set Enforcing").clicked() {
                set_selinux_enforcing(state);
            }
            
            if ui.button("🟡 Set Permissive").clicked() {
                set_selinux_permissive(state);
            }
        });
        
        ui.small("⚠️ Requires root access. Changes SELinux enforcement mode.");
    });
    
    ui.separator();
    
    // File Context Management
    ui.group(|ui| {
        ui.label(RichText::new("File Context Management").strong());
        
        ui.horizontal(|ui| {
            ui.label("File path:");
            ui.text_edit_singleline(&mut state.selinux_file_path);
        });
        
        ui.horizontal(|ui| {
            if ui.button("🔍 Get File Context").clicked() {
                get_file_selinux_context(state);
            }
            
            if ui.button("📝 Set File Context").clicked() {
                set_file_selinux_context(state);
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("New context:");
            ui.text_edit_singleline(&mut state.selinux_new_context);
        });
    });
    
    ui.separator();
    
    // Process Context Information
    ui.group(|ui| {
        ui.label(RichText::new("Process Context Information").strong());
        
        ui.horizontal(|ui| {
            ui.label("Process name/PID:");
            ui.text_edit_singleline(&mut state.selinux_process_query);
        });
        
        if ui.button("🔍 Get Process Contexts").clicked() {
            get_process_selinux_contexts(state);
        }
    });
    
    ui.separator();
    
    // SELinux Output Display
    if !state.selinux_output.is_empty() {
        ui.group(|ui| {
            ui.label(RichText::new("SELinux Information").strong());
            ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut state.selinux_output.as_str())
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace),
                    );
                });
                
            ui.horizontal(|ui| {
                if ui.button("📋 Copy to Clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = state.selinux_output.clone());
                }
                
                if ui.button("🗑️ Clear").clicked() {
                    state.selinux_output.clear();
                }
            });
        });
    }
}

fn get_selinux_status(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "getenforce"])
            .output();
            
        match output {
            Ok(result) => {
                let status = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== SELinux Status ===\n{}\n{}",
                    status.trim(),
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to get SELinux status: {}", e);
            }
        }
    }
}

fn get_selinux_contexts(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "ls", "-Z", "/"])
            .output();
            
        match output {
            Ok(result) => {
                let contexts = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== SELinux Contexts (Root Directory) ===\n{}\n{}",
                    contexts.trim(),
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to get SELinux contexts: {}", e);
            }
        }
    }
}

fn get_selinux_policy(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "cat", "/sys/fs/selinux/policy"])
            .output();
            
        match output {
            Ok(result) => {
                if result.stdout.is_empty() {
                    // Try alternative method
                    let alt_output = Command::new("adb")
                        .args(["-s", device_id, "shell", "ls", "-la", "/sys/fs/selinux/"])
                        .output();
                        
                    match alt_output {
                        Ok(alt_result) => {
                            let policy_info = String::from_utf8_lossy(&alt_result.stdout);
                            state.selinux_output = format!(
                                "=== SELinux Policy Information ===\n{}\n\nNote: Policy file is binary. Showing directory listing instead.",
                                policy_info.trim()
                            );
                        }
                        Err(e) => {
                            state.selinux_output = format!("Failed to get SELinux policy info: {}", e);
                        }
                    }
                } else {
                    state.selinux_output = "=== SELinux Policy ===\nPolicy file is binary and cannot be displayed as text.".to_string();
                }
            }
            Err(e) => {
                state.selinux_output = format!("Failed to access SELinux policy: {}", e);
            }
        }
    }
}

fn set_selinux_enforcing(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", "setenforce 1"])
            .output();
            
        match output {
            Ok(result) => {
                let success = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== Set SELinux to Enforcing ===\n{}\n{}",
                    if success.trim().is_empty() { "Command executed successfully" } else { success.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to set SELinux enforcing: {}", e);
            }
        }
    }
}

fn set_selinux_permissive(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", "setenforce 0"])
            .output();
            
        match output {
            Ok(result) => {
                let success = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== Set SELinux to Permissive ===\n{}\n{}",
                    if success.trim().is_empty() { "Command executed successfully" } else { success.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to set SELinux permissive: {}", e);
            }
        }
    }
}

fn get_file_selinux_context(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.selinux_file_path.trim().is_empty() {
            state.selinux_output = "Please enter a file path first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "ls", "-Z", &state.selinux_file_path])
            .output();
            
        match output {
            Ok(result) => {
                let context = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== File SELinux Context: {} ===\n{}\n{}",
                    state.selinux_file_path,
                    context.trim(),
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to get file SELinux context: {}", e);
            }
        }
    }
}

fn set_file_selinux_context(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.selinux_file_path.trim().is_empty() || state.selinux_new_context.trim().is_empty() {
            state.selinux_output = "Please enter both file path and new context.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("chcon {} {}", state.selinux_new_context, state.selinux_file_path)])
            .output();
            
        match output {
            Ok(result) => {
                let success = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== Set File SELinux Context ===\nFile: {}\nNew Context: {}\n{}\n{}",
                    state.selinux_file_path,
                    state.selinux_new_context,
                    if success.trim().is_empty() { "Command executed successfully" } else { success.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to set file SELinux context: {}", e);
            }
        }
    }
}

fn get_process_selinux_contexts(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.selinux_process_query.trim().is_empty() {
            state.selinux_output = "Please enter a process name or PID.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "ps", "-Z", "|", "grep", &state.selinux_process_query])
            .output();
            
        match output {
            Ok(result) => {
                let contexts = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.selinux_output = format!(
                    "=== Process SELinux Contexts for: {} ===\n{}\n{}",
                    state.selinux_process_query,
                    if contexts.trim().is_empty() { "No matching processes found" } else { contexts.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.selinux_output = format!("Failed to get process SELinux contexts: {}", e);
            }
        }
    }
}

fn show_systemd_tab(ui: &mut Ui, state: &mut AdbToolsState) {
    // Check systemd availability first
    ui.horizontal(|ui| {
        if ui.button("🔍 Check Systemd").clicked() {
            check_systemd_availability(state);
        }
        
        if ui.button("📊 System Status").clicked() {
            get_systemd_status(state);
        }
        
        if ui.button("🔄 Daemon Reload").clicked() {
            systemd_daemon_reload(state);
        }
    });
    
    ui.separator();
    
    // Service Management
    ui.group(|ui| {
        ui.label(RichText::new("Service Management").strong());
        
        ui.horizontal(|ui| {
            ui.label("Service name:");
            ui.text_edit_singleline(&mut state.systemd_service_name);
        });
        
        ui.horizontal(|ui| {
            if ui.button("▶️ Start").clicked() {
                systemd_start_service(state);
            }
            
            if ui.button("⏹️ Stop").clicked() {
                systemd_stop_service(state);
            }
            
            if ui.button("🔄 Restart").clicked() {
                systemd_restart_service(state);
            }
            
            if ui.button("🔃 Reload").clicked() {
                systemd_reload_service(state);
            }
        });
        
        ui.horizontal(|ui| {
            if ui.button("✅ Enable").clicked() {
                systemd_enable_service(state);
            }
            
            if ui.button("❌ Disable").clicked() {
                systemd_disable_service(state);
            }
            
            if ui.button("📋 Status").clicked() {
                systemd_service_status(state);
            }
        });
        
        ui.small("⚠️ Service management requires root access on most systems.");
    });
    
    ui.separator();
    
    // Unit Listing and Information
    ui.group(|ui| {
        ui.label(RichText::new("Unit Information").strong());
        
        ui.horizontal(|ui| {
            if ui.button("📜 List All Units").clicked() {
                systemd_list_units(state);
            }
            
            if ui.button("🔧 List Services").clicked() {
                systemd_list_services(state);
            }
            
            if ui.button("❌ List Failed").clicked() {
                systemd_list_failed(state);
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Filter units:");
            ui.text_edit_singleline(&mut state.systemd_unit_filter);
            if ui.button("🔍 Filter").clicked() {
                systemd_filter_units(state);
            }
        });
    });
    
    ui.separator();
    
    // System Control
    ui.group(|ui| {
        ui.label(RichText::new("System Control").strong());
        
        ui.horizontal(|ui| {
            if ui.button("📊 Show Boot Time").clicked() {
                systemd_analyze_time(state);
            }
            
            if ui.button("🐌 Show Blame").clicked() {
                systemd_analyze_blame(state);
            }
            
            if ui.button("🎯 Show Critical").clicked() {
                systemd_analyze_critical(state);
            }
        });
        
        ui.horizontal(|ui| {
            if ui.button("📋 Show Environment").clicked() {
                systemd_show_environment(state);
            }
            
            if ui.button("🔧 List Dependencies").clicked() {
                if !state.systemd_service_name.trim().is_empty() {
                    systemd_list_dependencies(state);
                } else {
                    state.systemd_output = "Please enter a service name first.".to_string();
                }
            }
        });
        
        ui.small("🔴 Advanced: Use with caution. Some operations may affect system stability.");
    });
    
    ui.separator();
    
    // Journal Management
    ui.group(|ui| {
        ui.label(RichText::new("Journal Management").strong());
        
        ui.horizontal(|ui| {
            if ui.button("📰 Show Journal").clicked() {
                systemd_show_journal(state);
            }
            
            if ui.button("🚨 Show Errors").clicked() {
                systemd_show_journal_errors(state);
            }
            
            if ui.button("🗓️ Today's Logs").clicked() {
                systemd_show_journal_today(state);
            }
        });
        
        ui.horizontal(|ui| {
            if ui.button("📊 Journal Size").clicked() {
                systemd_journal_disk_usage(state);
            }
            
            if ui.button("🧹 Vacuum Journal").clicked() {
                systemd_journal_vacuum(state);
            }
        });
    });
    
    ui.separator();
    
    // Systemd Output Display
    if !state.systemd_output.is_empty() {
        ui.group(|ui| {
            ui.label(RichText::new("Systemd Information").strong());
            ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut state.systemd_output.as_str())
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace),
                    );
                });
                
            ui.horizontal(|ui| {
                if ui.button("📋 Copy to Clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = state.systemd_output.clone());
                }
                
                if ui.button("🗑️ Clear").clicked() {
                    state.systemd_output.clear();
                }
            });
        });
    }
}

fn check_systemd_availability(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "which", "systemctl"])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                if !stdout.trim().is_empty() {
                    state.systemd_output = format!(
                        "=== Systemd Availability Check ===\n✅ Systemd is available at: {}\n",
                        stdout.trim()
                    );
                    
                    // Also check systemd version
                    let version_output = Command::new("adb")
                        .args(["-s", device_id, "shell", "systemctl", "--version"])
                        .output();
                        
                    if let Ok(version_result) = version_output {
                        let version = String::from_utf8_lossy(&version_result.stdout);
                        state.systemd_output.push_str(&format!("\n{}", version));
                    }
                } else {
                    state.systemd_output = format!(
                        "=== Systemd Availability Check ===\n❌ Systemd not found on this device.\nThis feature requires a Linux system with systemd.\n{}",
                        if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                    );
                }
            }
            Err(e) => {
                state.systemd_output = format!("Failed to check systemd availability: {}", e);
            }
        }
    }
}

fn get_systemd_status(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "status"])
            .output();
            
        match output {
            Ok(result) => {
                let status = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== System Status ===\n{}\n{}",
                    status,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to get system status: {}", e);
            }
        }
    }
}

fn systemd_daemon_reload(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", "systemctl daemon-reload"])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Daemon Reload ===\n{}\n{}",
                    if stdout.trim().is_empty() { "Daemon reload completed successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to reload daemon: {}", e);
            }
        }
    }
}

fn systemd_start_service(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("systemctl start {}", state.systemd_service_name)])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Start Service: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    if stdout.trim().is_empty() { "Service started successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to start service: {}", e);
            }
        }
    }
}

fn systemd_stop_service(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("systemctl stop {}", state.systemd_service_name)])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Stop Service: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    if stdout.trim().is_empty() { "Service stopped successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to stop service: {}", e);
            }
        }
    }
}

fn systemd_restart_service(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("systemctl restart {}", state.systemd_service_name)])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Restart Service: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    if stdout.trim().is_empty() { "Service restarted successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to restart service: {}", e);
            }
        }
    }
}

fn systemd_reload_service(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("systemctl reload {}", state.systemd_service_name)])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Reload Service: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    if stdout.trim().is_empty() { "Service configuration reloaded successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to reload service: {}", e);
            }
        }
    }
}

fn systemd_enable_service(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("systemctl enable {}", state.systemd_service_name)])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Enable Service: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    if stdout.trim().is_empty() { "Service enabled successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to enable service: {}", e);
            }
        }
    }
}

fn systemd_disable_service(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", &format!("systemctl disable {}", state.systemd_service_name)])
            .output();
            
        match output {
            Ok(result) => {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Disable Service: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    if stdout.trim().is_empty() { "Service disabled successfully" } else { stdout.trim() },
                    if !stderr.is_empty() { format!("Error: {}", stderr.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to disable service: {}", e);
            }
        }
    }
}

fn systemd_service_status(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_service_name.trim().is_empty() {
            state.systemd_output = "Please enter a service name first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "status", &state.systemd_service_name])
            .output();
            
        match output {
            Ok(result) => {
                let status = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Service Status: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    status,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to get service status: {}", e);
            }
        }
    }
}

fn systemd_list_units(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "list-units", "--no-pager"])
            .output();
            
        match output {
            Ok(result) => {
                let units = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== All Units ===\n{}\n{}",
                    units,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to list units: {}", e);
            }
        }
    }
}

fn systemd_list_services(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "list-units", "--type=service", "--no-pager"])
            .output();
            
        match output {
            Ok(result) => {
                let services = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== All Services ===\n{}\n{}",
                    services,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to list services: {}", e);
            }
        }
    }
}

fn systemd_list_failed(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "list-units", "--failed", "--no-pager"])
            .output();
            
        match output {
            Ok(result) => {
                let failed = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Failed Units ===\n{}\n{}",
                    if failed.trim().is_empty() { "No failed units found." } else { failed.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to list failed units: {}", e);
            }
        }
    }
}

fn systemd_filter_units(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        if state.systemd_unit_filter.trim().is_empty() {
            state.systemd_output = "Please enter a filter pattern first.".to_string();
            return;
        }
        
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "list-units", "--no-pager", "|", "grep", &state.systemd_unit_filter])
            .output();
            
        match output {
            Ok(result) => {
                let filtered = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Filtered Units: {} ===\n{}\n{}",
                    state.systemd_unit_filter,
                    if filtered.trim().is_empty() { "No matching units found." } else { filtered.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to filter units: {}", e);
            }
        }
    }
}

fn systemd_analyze_time(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemd-analyze", "time"])
            .output();
            
        match output {
            Ok(result) => {
                let time_info = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Boot Time Analysis ===\n{}\n{}",
                    time_info,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to analyze boot time: {}", e);
            }
        }
    }
}

fn systemd_analyze_blame(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemd-analyze", "blame"])
            .output();
            
        match output {
            Ok(result) => {
                let blame_info = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Boot Blame Analysis ===\n{}\n{}",
                    blame_info,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to analyze blame: {}", e);
            }
        }
    }
}

fn systemd_analyze_critical(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemd-analyze", "critical-chain"])
            .output();
            
        match output {
            Ok(result) => {
                let critical_info = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Critical Chain Analysis ===\n{}\n{}",
                    critical_info,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to analyze critical chain: {}", e);
            }
        }
    }
}

fn systemd_show_environment(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "show-environment"])
            .output();
            
        match output {
            Ok(result) => {
                let env_info = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Systemd Environment ===\n{}\n{}",
                    env_info,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to show environment: {}", e);
            }
        }
    }
}

fn systemd_list_dependencies(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "systemctl", "list-dependencies", &state.systemd_service_name])
            .output();
            
        match output {
            Ok(result) => {
                let deps = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Dependencies for: {} ===\n{}\n{}",
                    state.systemd_service_name,
                    deps,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to list dependencies: {}", e);
            }
        }
    }
}

fn systemd_show_journal(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "journalctl", "-n", "50", "--no-pager"])
            .output();
            
        match output {
            Ok(result) => {
                let journal = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Journal (Last 50 entries) ===\n{}\n{}",
                    journal,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to show journal: {}", e);
            }
        }
    }
}

fn systemd_show_journal_errors(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "journalctl", "-p", "err", "-n", "30", "--no-pager"])
            .output();
            
        match output {
            Ok(result) => {
                let errors = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Journal Errors (Last 30) ===\n{}\n{}",
                    if errors.trim().is_empty() { "No error entries found." } else { errors.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to show journal errors: {}", e);
            }
        }
    }
}

fn systemd_show_journal_today(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "journalctl", "--since", "today", "--no-pager"])
            .output();
            
        match output {
            Ok(result) => {
                let today_logs = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Today's Journal Entries ===\n{}\n{}",
                    if today_logs.trim().is_empty() { "No entries found for today." } else { today_logs.trim() },
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to show today's journal: {}", e);
            }
        }
    }
}

fn systemd_journal_disk_usage(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "journalctl", "--disk-usage"])
            .output();
            
        match output {
            Ok(result) => {
                let usage = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Journal Disk Usage ===\n{}\n{}",
                    usage,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to check journal disk usage: {}", e);
            }
        }
    }
}

fn systemd_journal_vacuum(state: &mut AdbToolsState) {
    if let Some(device_id) = &state.selected_device {
        let output = Command::new("adb")
            .args(["-s", device_id, "shell", "su", "-c", "journalctl --vacuum-time=1d"])
            .output();
            
        match output {
            Ok(result) => {
                let vacuum_result = String::from_utf8_lossy(&result.stdout);
                let error = String::from_utf8_lossy(&result.stderr);
                
                state.systemd_output = format!(
                    "=== Journal Vacuum (Keep 1 day) ===\n{}\n{}",
                    vacuum_result,
                    if !error.is_empty() { format!("Error: {}", error.trim()) } else { String::new() }
                );
            }
            Err(e) => {
                state.systemd_output = format!("Failed to vacuum journal: {}", e);
            }
        }
    }
}

// Helper functions
fn execute_adb_command(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("adb")
        .args(args)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("ADB command failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

fn extract_device_property(line: &str, property: &str) -> String {
    if let Some(start) = line.find(property) {
        let start = start + property.len();
        if let Some(end) = line[start..].find(' ') {
            line[start..start + end].to_string()
        } else {
            line[start..].to_string()
        }
    } else {
        String::new()
    }
}

fn extract_kb_value(memory_str: &str) -> Option<u64> {
    memory_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

fn add_time_series_data(state: &mut AdbToolsState) {
    if let Some(start_time) = state.time_series.start_time {
        let elapsed = start_time.elapsed().as_secs_f64();
        
        // Add CPU usage data point
        if let Some(cpu_load) = parse_cpu_load(&state.cpu_usage) {
            let data_point = DataPoint {
                timestamp: elapsed,
                value: cpu_load,
            };
            state.time_series.cpu_usage.push_back(data_point);
            
            // Keep only max_points
            while state.time_series.cpu_usage.len() > state.time_series.max_points {
                state.time_series.cpu_usage.pop_front();
            }
        }
        
        // Add memory usage data point
        if let Some(memory_usage) = parse_memory_usage(&state.memory_info) {
            let data_point = DataPoint {
                timestamp: elapsed,
                value: memory_usage,
            };
            state.time_series.memory_usage.push_back(data_point);
            
            while state.time_series.memory_usage.len() > state.time_series.max_points {
                state.time_series.memory_usage.pop_front();
            }
        }
        
        // Add battery level data point
        if let Some(battery_level) = parse_battery_level(&state.battery_info) {
            let data_point = DataPoint {
                timestamp: elapsed,
                value: battery_level,
            };
            state.time_series.battery_level.push_back(data_point);
            
            while state.time_series.battery_level.len() > state.time_series.max_points {
                state.time_series.battery_level.pop_front();
            }
        }
        
        // Add battery temperature data point
        if let Some(battery_temp) = parse_battery_temperature(&state.battery_info) {
            let data_point = DataPoint {
                timestamp: elapsed,
                value: battery_temp,
            };
            state.time_series.battery_temperature.push_back(data_point);
            
            while state.time_series.battery_temperature.len() > state.time_series.max_points {
                state.time_series.battery_temperature.pop_front();
            }
        }
    }
}

fn trim_time_series_data(state: &mut AdbToolsState) {
    let max_points = state.time_series.max_points;
    
    // Trim CPU usage data
    while state.time_series.cpu_usage.len() > max_points {
        state.time_series.cpu_usage.pop_front();
    }
    
    // Trim memory usage data
    while state.time_series.memory_usage.len() > max_points {
        state.time_series.memory_usage.pop_front();
    }
    
    // Trim battery level data
    while state.time_series.battery_level.len() > max_points {
        state.time_series.battery_level.pop_front();
    }
    
    // Trim battery temperature data
    while state.time_series.battery_temperature.len() > max_points {
        state.time_series.battery_temperature.pop_front();
    }
}

fn clear_plot_data(state: &mut AdbToolsState) {
    state.time_series.cpu_usage.clear();
    state.time_series.memory_usage.clear();
    state.time_series.battery_level.clear();
    state.time_series.battery_temperature.clear();
    state.time_series.start_time = if state.monitoring_enabled {
        Some(Instant::now())
    } else {
        None
    };
}

fn parse_cpu_load(cpu_usage: &str) -> Option<f64> {
    // Parse "Load: 1.23 0.45 0.67 (1m 5m 15m) | 8 cores" format
    if let Some(start) = cpu_usage.find("Load: ") {
        let load_part = &cpu_usage[start + 6..];
        if let Some(space_pos) = load_part.find(' ') {
            return load_part[..space_pos].parse().ok();
        }
    }
    None
}

fn parse_memory_usage(memory_info: &HashMap<String, String>) -> Option<f64> {
    // Parse memory usage percentage from "Memory Usage" field
    if let Some(usage_str) = memory_info.get("Memory Usage") {
        if let Some(percent_pos) = usage_str.find('%') {
            return usage_str[..percent_pos].parse().ok();
        }
    }
    None
}

fn parse_battery_level(battery_info: &HashMap<String, String>) -> Option<f64> {
    // Parse battery level from "Battery Level" field
    if let Some(level_str) = battery_info.get("Battery Level") {
        if let Some(percent_pos) = level_str.find('%') {
            return level_str[..percent_pos].parse().ok();
        }
    }
    None
}

fn parse_battery_temperature(battery_info: &HashMap<String, String>) -> Option<f64> {
    // Parse battery temperature from "Temperature" field
    if let Some(temp_str) = battery_info.get("Temperature") {
        if let Some(degree_pos) = temp_str.find('°') {
            return temp_str[..degree_pos].parse().ok();
        }
    }
    None
}
