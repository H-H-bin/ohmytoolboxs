# ADB Tools Test Guide

This document provides guidance for testing the ADB tools functionality in OhMyToolboxs.

## Prerequisites

1. **Install ADB**: Download Android SDK Platform Tools or install ADB standalone
2. **Add to PATH**: Ensure `adb` command is available from command line
3. **Enable USB Debugging**: On your Android device:
   - Go to Settings > About Phone
   - Tap "Build Number" 7 times to enable Developer Options
   - Go to Settings > Developer Options
   - Enable "USB Debugging"

## Testing Steps

### 1. Test ADB Installation
```bash
adb version
```
Should display ADB version information.

### 2. Connect Android Device
```bash
adb devices
```
Should list connected devices. If "unauthorized", check device for USB debugging authorization dialog.

### 3. Test OhMyToolboxs ADB Tools

1. **Launch OhMyToolboxs**
   ```bash
   cargo run
   ```

2. **Navigate to ADB Tools**
   - Click on "🤖 ADB Tools" in the sidebar

3. **Device Management**
   - Click "🔄 Refresh Devices" to scan for connected devices
   - Select a device from the dropdown

4. **Test Features**
   - **Device Information**: Click "📊 Get Device Info" to display device properties
   - **Device Monitor**: Click "📈 Device Monitor" and then "▶️ Start Monitoring" for real-time stats
   - **App Management**: Click "📦 List Packages" to see installed apps
   - **File Operations**: Try listing a directory like "/sdcard/"
   - **Shell Commands**: Execute simple commands like "ls /sdcard/" or "getprop"

## Common Issues

### ADB Not Found
- **Error**: "adb command not found" or similar
- **Solution**: Install Android SDK Platform Tools and add to PATH

### No Devices Listed
- **Error**: Device not appearing in refresh
- **Solutions**:
  - Check USB debugging is enabled
  - Try different USB cable
  - Authorize computer on device
  - Run `adb kill-server && adb start-server`

### Device Unauthorized
- **Error**: Device shows as "unauthorized"
- **Solution**: Check device screen for authorization dialog and tap "Always allow"

## Example Commands to Test

### Device Info
- Model: `getprop ro.product.model`
- Android Version: `getprop ro.build.version.release`
- Battery: `dumpsys battery`

### File Operations
- List SD card: `ls /sdcard/`
- Check storage: `df -h`

### App Management
- List all packages: `pm list packages`
- List system apps: `pm list packages -s`
- List third-party apps: `pm list packages -3`

### Device Monitoring
- CPU Load: `cat /proc/loadavg`
- Memory Info: `cat /proc/meminfo`
- Process List: `ps` or `ps -o PID,NAME,CPU,RSS,USER,STAT`
- Battery Status: `dumpsys battery`
- Thermal Status: `cat /sys/class/thermal/thermal_zone*/temp`
- Network Stats: `cat /proc/net/dev`

## Notes

- Some features may require root access on the device
- Screen recording functionality is basic in this implementation
- Logcat may produce large amounts of output - use filters when possible
- Port forwarding requires available ports on both computer and device
