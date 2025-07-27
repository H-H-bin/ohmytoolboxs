# Configuration Management in OhMyToolboxs

OhMyToolboxs includes a comprehensive configuration system that persists all your tool settings and preferences automatically. The application supports **portable mode** by default, making it perfect for USB drives and portable installations.

## Features

- **Portable by Default**: Configuration stored next to executable for maximum portability
- **Flexible Storage Options**: Choose between portable, system, or custom locations
- **Automatic Settings Persistence**: All your tool configurations are automatically saved when you close the application
- **Manual Save/Load**: You can manually save and load settings through the Configuration dialog
- **Tool-Specific Settings**: Each tool category maintains its own settings independently
- **Cross-Platform**: Configuration files work across Windows, macOS, and Linux
- **Human-Readable Format**: Configuration is stored in TOML format for easy manual editing

## Configuration Storage Modes

### 📦 Portable Mode (Default)
- Configuration stored as `config.toml` next to the executable
- Perfect for USB drives and portable installations
- No dependency on user profile directories
- Easy to backup with the application
- **Automatically selected** if the application can write to its own directory

### 🏠 System Mode
- Configuration stored in the user's standard config directory
- Follows OS conventions for application data
- Separate from the application installation

### 📁 Custom Mode
- Configuration stored in a user-specified location
- Full control over where settings are saved
- Useful for shared configurations or network drives

## Configuration File Locations

### Portable Mode
- **All Platforms**: `config.toml` in the same directory as the executable

### System Mode
- **Windows**: `%APPDATA%\ohmytoolboxs\config.toml`
- **macOS**: `~/Library/Application Support/ohmytoolboxs/config.toml`
- **Linux**: `~/.config/ohmytoolboxs/config.toml`

### Custom Mode
- **All Platforms**: User-specified path

## What Gets Saved

### Application Settings
- Dark/light mode preference
- Sidebar width
- Window size (width/height)
- Tool category visibility preferences

### ADB Tools Settings
- Selected device
- Package filter
- APK path for installations
- Local and remote paths for file operations
- Shell command history
- Logcat filter settings
- Screenshot and recording paths
- Port forwarding configurations
- Monitor interval settings
- Plot visibility preferences
- SELinux management settings
- Systemd management settings
- Function visibility (which ADB functions to show/hide)

### Text Tools Settings
- Case conversion text
- Encoder/decoder text
- Hash input text
- JSON formatting text
- Regex patterns and test text
- Find/replace patterns

### Network Tools Settings
- Default ping host
- Port scan ranges and targets
- DNS lookup hosts
- HTTP request configurations (URL, method, headers, body)

### File Tools Settings
- Selected directories
- Search patterns
- File extension filters
- Replace patterns
- Backup source and destination paths

### System Tools Settings
- Process and service filters
- Log file paths and filters
- Command history (with configurable size limit)

### Developer Tools Settings
- Git repository paths
- Commit messages and branch names
- Database connection strings
- SQL queries
- API endpoints and keys
- Docker image and container configurations

## Using the Configuration System

### Accessing Configuration Settings

1. Click on **File** menu in the top menu bar
2. Select **Settings** → **Configuration**
3. The Configuration Settings dialog will open

### Manual Save/Load Operations

- **💾 Save Settings**: Immediately saves all current settings to the config file
- **🔄 Load Settings**: Reloads settings from the config file (useful if you edited it manually)
- **🗑️ Reset to Defaults**: Resets all settings to their default values

### Automatic Behavior

- Settings are automatically loaded when the application starts
- Settings are automatically saved when the application closes
- Tool configurations are preserved between sessions

## Manual Configuration Editing

You can manually edit the `config.toml` file using any text editor. The file uses TOML format which is human-readable and easy to modify.

### Example Configuration Structure

```toml
[app_settings]
dark_mode = true
sidebar_width = 250.0
window_width = 1200.0
window_height = 800.0

[tool_settings.adb_tools]
selected_device = "emulator-5554"
package_filter = "com.example"
monitor_interval = 2.0
show_plots = true

[tool_settings.text_tools]
case_conversion_text = "Hello World"
regex_pattern = "\\d+"
```

### Configuration Sections

- `[app_settings]`: Global application preferences
- `[tool_settings.adb_tools]`: ADB-specific settings
- `[tool_settings.text_tools]`: Text manipulation tool settings
- `[tool_settings.network_tools]`: Network diagnostic tool settings
- `[tool_settings.file_tools]`: File operation tool settings
- `[tool_settings.system_tools]`: System monitoring tool settings
- `[tool_settings.dev_tools]`: Developer tool settings

## Best Practices

1. **Use the UI for Changes**: While manual editing is supported, it's recommended to use the UI to change settings
2. **Backup Your Config**: Consider backing up your `config.toml` file, especially if you have complex custom configurations
3. **Validate Manual Edits**: If you manually edit the config file, use the "Load Settings" button to ensure the changes are valid
4. **Reset if Corrupted**: If the config file becomes corrupted, use "Reset to Defaults" to restore working settings

## Troubleshooting

### Configuration File Not Loading
- Check that the config file exists in the correct location
- Verify the TOML syntax is valid
- Use "Reset to Defaults" if the file is corrupted

### Settings Not Persisting
- Ensure the application has write permissions to the config directory
- Check that you're not running multiple instances of the application
- Verify the config file is not read-only

### Manual Edits Not Taking Effect
- Click "Load Settings" after manual edits
- Restart the application to ensure all changes are applied
- Check for syntax errors in the TOML file

## Migration and Backup

### Backing Up Settings
```bash
# Copy your config file to a backup location
cp ~/.config/ohmytoolboxs/config.toml ~/ohmytoolboxs_backup.toml
```

### Restoring Settings
```bash
# Restore from backup
cp ~/ohmytoolboxs_backup.toml ~/.config/ohmytoolboxs/config.toml
```

### Sharing Settings
You can share your configuration with others by copying the `config.toml` file. This is useful for teams that want to standardize tool configurations.

## Managing Configuration Modes

### Switching Between Modes

1. **Access Configuration Settings**:
   - Click **File** → **Settings** → **Configuration**
   
2. **Choose Your Mode**:
   - **📦 Switch to Portable Mode**: Store config next to executable
   - **🏠 Switch to System Mode**: Store in user's standard config directory  
   - **📁 Choose Custom Location**: Specify a custom path

3. **Automatic Migration**:
   - When switching modes, your current settings are automatically copied to the new location
   - No configuration is lost during the transition

### Mode Selection Logic

The application automatically determines the best configuration mode on first run:

1. **Check for Existing Portable Config**: If `config.toml` exists next to the executable, use it
2. **Test Write Permissions**: If the executable directory is writable, use portable mode
3. **Fallback to System Mode**: If portable mode isn't possible, use system directory

### Recommendations

- **📦 Portable Mode**: Best for USB drives, portable installations, or when you want everything contained
- **🏠 System Mode**: Best for permanent installations where you want OS-standard behavior
- **📁 Custom Mode**: Best for shared configurations, network drives, or specific organizational needs

---

The configuration system ensures that OhMyToolboxs remembers your preferences and tool settings, making your workflow more efficient and personalized.
