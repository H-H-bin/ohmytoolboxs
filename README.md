# OhMyToolboxs - Android Development Tools

A comprehensive Android development toolbox application built with Rust and egui, featuring ADB and Fastboot tools for complete Android device management and firmware operations.

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

### ⚡ Android Fastboot Tools

**Device Management**
- Detect and connect to Android devices in fastboot mode
- Device information and variable inspection
- Fastboot availability checking and tool validation

**Flash Operations**
- Flash firmware images to specific partitions (boot, recovery, system, etc.)
- Support for standard Android partitions with safety validations
- Real-time progress indication for flash operations
- Comprehensive error handling and result reporting

**Bootloader Management**
- Unlock and lock bootloader operations with safety warnings
- Bootloader status checking and verification
- Secure handling of bootloader state changes

**Partition Operations**
- Erase partitions safely (cache, userdata, etc.)
- Format partitions with proper file system setup
- Partition management with comprehensive safety checks

**System Operations**
- Boot from image files without permanent flashing
- Flash complete firmware packages from update ZIP files
- Reboot to different modes (system, bootloader, recovery, fastboot)
- System-level operations with proper validation

**Safety Features**
- ⚠️ **Comprehensive warnings** for destructive operations
- **Data loss prevention** with multiple confirmation steps
- **Device compatibility** checking before operations
- **Operation logging** and detailed error reporting

### 📱 Qualcomm Download Tool (QDL)

**Device Management**
- Detect and connect to Qualcomm devices in EDL/9008 mode
- Device information and communication status monitoring
- Sahara and Firehose protocol support

**Flash Operations**
- Flash firmware images to specific partitions with progress tracking
- Support for META file-based flashing operations
- Comprehensive safety validations and error handling
- Real-time progress indication for flash operations

**Partition Management**
- List available partitions with detailed information
- Set partitions as bootable for system recovery
- Erase specific partitions with safety confirmations
- LUN (Logical Unit Number) support for storage device selection (0-7)

**Storage Operations**
- Dump partition contents to files for backup purposes
- Storage device recovery and management
- File-based storage operations with progress monitoring

**Memory Operations**
- Memory peek operations to read specific memory addresses
- Memory poke operations to write data to memory locations
- Memory dump collection for debugging and analysis
- Address range validation and safety checks

**System Operations**
- Reboot device to different modes (normal, EDL, fastboot)
- Load programmer files for low-level operations
- NOP (No Operation) commands for testing communication
- System-level debugging and recovery operations

### 🧠 Qualcomm RAM Dump Tool (QRamdump)

**Device Management**
- Detect and connect to crashed Qualcomm devices
- Device crash status monitoring and validation
- Communication interface management for dump collection

**Memory Dump Collection**
- **Full System Dump**: Complete memory image collection
- **Partial Dump**: Selective memory region dumping
- **Selective Dump**: Custom memory range specification
- **Kernel Only**: Kernel space memory collection
- **User Only**: User space memory collection
- Progress tracking and estimated time calculation

**Crash Analysis**
- **Log Extraction**: System logs and crash logs analysis
- **Stack Trace**: Call stack analysis and debugging information
- **Exception Details**: Exception type and cause analysis
- **Memory Analysis**: Memory corruption and leak detection
- Comprehensive crash report generation

**File Management**
- Dump file compression and organization
- Export dumps in various formats
- File naming with timestamps and device information
- Storage management and cleanup utilities

**System Information**
- Hardware configuration and device specifications
- Software version and build information
- System state analysis at time of crash
- Environmental data collection (temperature, voltage, etc.)

**Safety Features**
- ⚠️ **Non-destructive operations** - Read-only memory access
- **Device state preservation** during dump collection
- **Automatic error recovery** if communication fails
- **Progress monitoring** with ability to pause/resume operations

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
- **Android Debug Bridge (ADB)** - Required for ADB Tools functionality
  - Install via Android SDK Platform Tools or standalone ADB
  - Ensure `adb` command is available in your system PATH
- **Android Fastboot** - Required for Fastboot Tools functionality
  - Usually included with Android SDK Platform Tools
  - Ensure `fastboot` command is available in your system PATH
  - **Important**: Fastboot requires devices to be in fastboot/bootloader mode
- **Qualcomm Tools** - Required for QDL and QRamdump Tools functionality
  - QDL (Qualcomm Download Tool) support for EDL/9008 mode operations
  - QRamdump tool support for memory dump collection and crash analysis
  - **Important**: These tools require Qualcomm devices in specific modes (EDL for QDL, crashed state for QRamdump)

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
│   ├── dev_tools.rs  # Developer utilities
│   ├── qdl_tools.rs  # Qualcomm Download Tool (QDL) for EDL/9008 mode
│   └── qramdump_tools.rs # Qualcomm RAM Dump Tool for crash analysis
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
- **QDL Tools Enhancements**: Advanced partition management and custom programmer support
- **QRamdump Tools Enhancements**: Real-time crash monitoring and automated analysis reports
- **Qualcomm Tool Integration**: Batch operations and configuration management for QDL/QRamdump workflows

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Acknowledgments

- Built with the amazing [egui](https://github.com/emilk/egui) immediate mode GUI library
- Inspired by various developer toolbox applications
- Thanks to the Rust community for excellent crates and documentation
