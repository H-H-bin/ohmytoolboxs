# Device Monitoring Features

This document describes the comprehensive device monitoring capabilities added to the ADB Tools in OhMyToolboxs, including real-time data visualization through interactive plots.

## 📈 Real-time Device Monitor with Plotting

### Overview
The Device Monitor provides real-time visibility into Android device performance, system resources, and running processes. It features interactive time-series plots that visualize performance trends over time, making it easy to identify patterns and performance issues.

### Features

#### 📊 Interactive Performance Plots
- **Real-time Visualization**: Time-series plots that update automatically during monitoring
- **CPU Load Plot**: Visual representation of CPU load average over time
- **Memory Usage Plot**: Memory usage percentage trends with color-coded visualization
- **Battery Level Plot**: Battery charge level changes over time
- **Battery Temperature Plot**: Thermal monitoring with temperature trends
- **Configurable Data Points**: Maintains up to 100 data points for smooth visualization
- **Plot Controls**: Toggle plots on/off, clear historical data, view data point counts

#### 🖥️ System Performance Monitoring
- **CPU Usage**: Real-time CPU load average (1m, 5m, 15m intervals)
- **CPU Cores**: Detection of CPU core count
- **Memory Statistics**: 
  - Total Memory
  - Free Memory
  - Available Memory
  - Buffers and Cache
  - Swap usage
  - Memory usage percentage calculation

#### 🔋 Battery & Thermal Monitoring
- **Battery Level**: Current charge percentage
- **Battery Temperature**: Real-time temperature monitoring
- **Battery Voltage**: Current voltage readings
- **Battery Health**: Health status reporting
- **Charging Status**: AC/USB power state
- **Thermal Zones**: System thermal sensor readings

#### 🌐 Network Statistics
- **Interface Monitoring**: WiFi (wlan0), Mobile (rmnet0), Ethernet (eth0)
- **Data Transfer**: Real-time RX/TX byte counters
- **Bandwidth Usage**: Formatted byte display (B, KB, MB, GB, TB)

#### ⚙️ Process Monitor
- **Process List**: All running processes with detailed information
- **Process Information**:
  - Process ID (PID)
  - Process Name
  - CPU Usage Percentage
  - Memory Usage (RSS)
  - User/Owner
  - Process State
- **Process Management**:
  - Filter processes by name or PID
  - Kill processes directly from the interface
  - Real-time process list updates
- **Performance Optimization**: Limited to 50 processes for UI performance

### 📱 Device Management

#### Auto-Connect Feature
- **Single Device Auto-Selection**: When only one device is connected, it's automatically selected
- **Initial Refresh**: Device list is automatically refreshed when ADB Tools are first opened
- **Status Indicators**: Visual feedback shows connection status:
  - 🔗 Green "Auto-connected to single device" when single device is auto-selected
  - 📱 Device count display when multiple devices are available
  - ⚠ Yellow warning when no devices are found
- **Smart Selection Management**: 
  - Maintains current selection when multiple devices remain connected
  - Clears selection automatically when previously selected device disconnects
  - Re-selects automatically when device count returns to one

#### Manual Device Selection
- **Device Dropdown**: Manual selection available when multiple devices are connected
- **Device Information**: Shows device ID, status, and model in selection list
- **Refresh Control**: Manual refresh button updates device list on demand
- **Connection Validation**: Ensures selected device is still available before operations

### 🎛️ Monitoring Controls

#### Start/Stop Monitoring
- **Toggle Button**: Start/Stop real-time monitoring
- **Update Interval**: Configurable from 1-10 seconds
- **Manual Update**: Force immediate data refresh
- **Auto Refresh**: Automatic UI updates when monitoring is active

#### Real-time Updates
- **Timer System**: Uses Instant-based timing for precise intervals
- **UI Refresh**: Automatic context repainting for smooth updates
- **Background Processing**: Non-blocking ADB command execution
- **State Persistence**: Monitoring preferences saved with application state

#### 📈 Plot Visualization Features
- **Time-Series Data**: Automatically collects and stores performance data over time
- **Color-Coded Lines**: Each metric has a distinct color for easy identification
- **Zoom and Pan**: Interactive plot controls for detailed analysis
- **Configurable Buffer**: User-adjustable data point limit (10-10,000, default: 1000)
- **Plot Toggle**: Show/hide plots independently from raw data monitoring
- **Clear Data**: Reset plot history while maintaining current monitoring session
- **Multi-Metric Display**: View CPU, memory, battery level, and temperature simultaneously
- **Dynamic Scaling**: Automatically adjust buffer size with real-time data trimming

### 📊 Data Sources

#### CPU Information
```bash
# CPU load average
cat /proc/loadavg

# CPU core count
cat /proc/cpuinfo
```

#### Memory Information
```bash
# Detailed memory statistics
cat /proc/meminfo
```

#### Battery Information
```bash
# Battery status and health
dumpsys battery
```

#### Thermal Information
```bash
# Thermal zone temperatures
cat /sys/class/thermal/thermal_zone0/temp
```

#### Network Statistics
```bash
# Network interface statistics
cat /proc/net/dev
```

#### Process Information
```bash
# Detailed process list
ps -o PID,NAME,CPU,RSS,USER,STAT

# Fallback simple process list
ps
```

### 🔧 Technical Implementation

#### Architecture
- **State Management**: Comprehensive state tracking with serialization support
- **Error Handling**: Graceful fallbacks for missing data or permissions
- **Performance**: Efficient data parsing and UI rendering
- **Cross-Platform**: Works on all ADB-supported platforms

#### Data Processing
- **Memory Calculations**: Automatic percentage calculations
- **Temperature Conversion**: Proper unit conversion (millidegrees to Celsius)
- **Voltage Conversion**: Millivolt to volt conversion
- **Byte Formatting**: Human-readable size formatting

#### UI Components
- **Collapsible Sections**: Organized data presentation
- **Grid Layouts**: Structured data display
- **Scrollable Areas**: Handle large data sets
- **Real-time Indicators**: Visual feedback for monitoring status

### 🚀 Usage Instructions

1. **Connect Device**: Ensure Android device is connected via ADB
2. **Auto-Selection**: Single devices are automatically selected; multiple devices require manual selection
3. **Open Monitor**: Expand "📈 Device Monitor" section
4. **Start Monitoring**: Click "▶️ Start Monitoring"
5. **Configure Interval**: Adjust update frequency (1-10 seconds)
6. **View Data**: Monitor real-time statistics across all categories
7. **Manage Processes**: Filter, select, and kill processes as needed
8. **Stop Monitoring**: Click "⏹️ Stop Monitoring" when done

### ⚠️ Important Notes

#### Permissions
- Some data may require root access on the device
- Process killing requires appropriate permissions
- Thermal sensors may not be available on all devices

#### Performance
- Higher update frequencies increase system load
- Process list is limited to 50 entries for UI performance
- Network and battery data updates consume minimal resources

#### Compatibility
- Works with all ADB-compatible Android devices
- Graceful degradation for missing features or permissions
- Fallback mechanisms for unsupported commands

### 🔍 Troubleshooting

#### No Data Displayed
- Verify device is properly connected and authorized
- Check that ADB debugging is enabled
- Try manual "Update Now" to test connectivity

#### Missing Information
- Some features require specific Android versions
- Root access may be needed for certain data
- Different devices may have varying sensor availability

#### Performance Issues
- Reduce update interval if system becomes slow
- Use process filtering to reduce data processing
- Close other ADB-using applications to avoid conflicts

This monitoring system provides comprehensive insight into Android device performance and is essential for developers, testers, and system administrators working with Android devices.
