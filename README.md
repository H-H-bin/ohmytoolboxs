# OhMyToolboxs - ADB Tools �

A comprehensive Android Debug Bridge (ADB) toolbox application built with Rust and egui, designed specifically for Android development and device management.

## Features

### 🤖 Android Debug Bridge (ADB) Tools

**Device Management**
- Detect and connect to Android devices via ADB with auto-connect for single devices
- Real-time device status monitoring and connection management

**Real-time Monitoring**
- Monitor CPU usage, memory, battery, thermal status in real-time
- Interactive time-series plots with configurable data points (10-10,000, default: 1000)
- Process monitoring with CPU and memory usage, kill processes

**Application Management**
- List installed applications with filtering
- Install and uninstall Android applications (APK management)
- Package management and application information

**File Operations**
- Push/pull files between computer and Android device
- Directory browsing and file management
- File transfer with progress tracking

**Development Tools**
- Execute shell commands on Android devices
- Logcat viewer with real-time filtering and search
- Screen capture (screenshots) and screen recording
- Port forwarding between computer and device

**Advanced Features**
- **SELinux Management**: Manage SELinux policies, contexts, and security settings
  - Status monitoring (Enforcing/Permissive mode checking)
  - Mode control (switch between enforcing and permissive modes)
  - Context inspection for files and directories
  - Context management and modification
  - Process context viewing
  - **Policy Information**: Access SELinux policy details and directory listings
- **Systemd Management**: Manage systemd services, units, and system daemon control
  - **Service Control**: Start, stop, restart, reload, enable, and disable services
  - **System Status**: Check overall systemd status and boot performance
  - **Unit Management**: List all units, services, failed units with filtering
  - **System Analysis**: Boot time analysis, blame reports, and critical chain inspection
  - **Journal Management**: View system logs, errors, and manage journal storage
  - **Dependency Tracking**: List service dependencies and system environment

## Technical Details

- **Framework**: Built with [egui](https://github.com/emilk/egui) for immediate mode GUI
- **Language**: Rust 2021 edition
- **Platform**: Cross-platform support (Windows, macOS, Linux)
- **Persistence**: Automatic saving of application state and preferences

## User Interface Features

### 🎨 Customizable Interface
- **Dark/Light Mode**: Toggle between dark and light themes via View menu
- **Tool Category Management**: Show/hide specific tool categories through File → Settings → Tool Categories
- **Resizable Sidebar**: Adjustable sidebar width for optimal workspace organization
- **Search Functionality**: Quick search through tool categories and descriptions
- **Persistent Settings**: All preferences are automatically saved and restored

### ⚙️ Settings Menu
Access via **File → Settings** in the top menu bar:

- **Tool Categories**: Configure which tool categories appear in the sidebar
  - Individual toggle controls for each category (Text Tools, System Tools, Network Tools, etc.)
  - "Select All" and "Deselect All" buttons for quick configuration
  - Real-time sidebar updates without restart required
  - Automatic selection clearing when hiding the currently selected category

### 🔍 Search and Navigation
- **Category Search**: Filter visible categories by name or description
- **Responsive Design**: Automatically adapts to window resizing
- **Keyboard Friendly**: Full keyboard navigation support

## Building and Running

### Prerequisites

- Rust 1.70+ (with Cargo)
- Git
- Android Debug Bridge (ADB) - Required for ADB Tools functionality
  - Install via Android SDK Platform Tools or standalone ADB
  - Ensure `adb` command is available in your system PATH

### Build from Source
```bash
git clone <repository-url>
cd ohmytoolboxs
cargo build --release
```

### Run the Application
```bash
cargo run
```

### Development Mode
```bash
cargo run
# or for debug logging
RUST_LOG=debug cargo run
```

## Project Structure

```
src/
├── main.rs           # Application entry point
├── app.rs            # Main application logic and state management
├── tools/            # Tool implementations
│   ├── mod.rs        # Tool category definitions
│   ├── text_tools.rs # Text manipulation utilities
│   ├── system_tools.rs # System information and process tools
│   ├── network_tools.rs # Network diagnostic tools
│   ├── file_tools.rs # File management utilities
│   └── dev_tools.rs  # Developer utilities
└── ui/               # User interface components
    ├── mod.rs        # UI module definitions
    ├── sidebar.rs    # Navigation sidebar
    └── content.rs    # Main content area
```

## Dependencies

- `egui` - Immediate mode GUI framework
- `eframe` - egui framework for native applications
- `egui_extras` - Additional egui components and loaders
- `serde` - Serialization framework for state persistence
- `serde_json` - JSON parsing and formatting
- `base64` - Base64 encoding/decoding
- `log` + `env_logger` - Logging infrastructure

## Features in Development

- Enhanced regex testing with pattern highlighting
- Complete hash generator implementation (MD5, SHA256, SHA512)
- Network tools with actual ping and port scanning
- Color tools with palette generation and format conversion
- File compression and decompression utilities
- More text encoding formats (hex, binary, etc.)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Acknowledgments

- Built with the amazing [egui](https://github.com/emilk/egui) immediate mode GUI library
- Inspired by various developer toolbox applications
- Thanks to the Rust community for excellent crates and documentation
