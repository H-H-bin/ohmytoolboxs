# Fastboot Tools Test Guide

This document provides guidance for testing the Fastboot tools functionality in OhMyToolboxs.

## Prerequisites

### 1. Install Android Platform Tools
Download Android Platform Tools that include fastboot:
- **Windows**: Download from [Android Developer website](https://developer.android.com/studio/releases/platform-tools)
- Extract to a folder and add to PATH
- Or place `fastboot.exe` in the same directory as OhMyToolboxs

### 2. Device Requirements
- Android device with unlockable bootloader
- USB Debugging enabled in Developer Options
- Device booted into fastboot mode:
  ```bash
  adb reboot bootloader
  ```
  Or manually: Power + Volume Down (varies by device)

### 3. Drivers (Windows)
- Install proper USB drivers for your device
- Google USB drivers or device-specific drivers

## Testing Steps

### 1. Test Fastboot Installation
```bash
fastboot --version
```
Should display fastboot version information.

### 2. Connect Device in Fastboot Mode
```bash
fastboot devices
```
Should list your device. If empty, check:
- Device is in fastboot mode
- USB drivers are installed
- USB cable supports data transfer

### 3. Test OhMyToolboxs Fastboot Tools

1. **Launch OhMyToolboxs**
   ```bash
   cargo run
   ```

2. **Navigate to Fastboot Tools**
   - Click on "⚡ Fastboot Tools" in the sidebar

3. **Device Management**
   - Click "🔄 Refresh Devices" to scan for connected devices
   - Select a device from the dropdown

4. **Test Features**
   - **Device Information**: Click "📊 Get Device Info" to display device properties
   - **Check Fastboot**: Click "🔍 Check Fastboot" to verify fastboot availability
   - **Bootloader Status**: Click "🔍 Check Lock Status" to see if bootloader is unlocked

## Common Issues

### Device Not Detected
- Ensure device is in fastboot mode (not normal boot or recovery)
- Check USB cable and drivers
- Try different USB ports
- Run `fastboot devices` in terminal to verify

### Permission Errors
- On Linux/macOS: Run with sudo if needed
- On Windows: Run as administrator if needed

### Fastboot Not Found
- Add Android Platform Tools to system PATH
- Or place fastboot executable in application directory

## Example Commands to Test

### Basic Commands (safe to test):
```bash
fastboot devices
fastboot getvar product
fastboot getvar version-bootloader
fastboot getvar unlocked
```

### Advanced Commands (⚠️ USE WITH CAUTION):
```bash
# These can modify your device - only use if you know what you're doing
fastboot erase cache          # Safe on most devices
fastboot format userdata      # ⚠️ ERASES ALL USER DATA
fastboot flash boot boot.img  # ⚠️ Can brick device if wrong image
```

## Safety Notes

⚠️ **IMPORTANT WARNINGS**:
- **Bootloader unlock will erase all data**
- **Wrong firmware can permanently damage device**
- **Always have working recovery image**
- **Know how to enter download/recovery mode**
- **Keep device charged (>50%)**

### Recommended Test Sequence:
1. Device Info ✅ (Safe)
2. Get Variables ✅ (Safe)
3. Check Lock Status ✅ (Safe)
4. Erase Cache ⚠️ (Usually safe)
5. Flash Operations ❌ (Only with correct images)
6. Bootloader Unlock ❌ (Only if intended)

## Device Recovery

If something goes wrong:
1. **Soft brick**: Use recovery mode to flash correct firmware
2. **Hard brick**: Use download mode (Samsung) or EDL mode (Qualcomm)
3. **Bootloader issues**: Contact device manufacturer

## Notes

- Test with a non-critical device first
- Always backup important data before testing
- Different manufacturers have different fastboot implementations
- Some features may not work on all devices
- Custom ROMs may behave differently than stock firmware

## Supported Operations

### ✅ Currently Implemented:
- Device detection and info
- Variable reading (product, version, etc.)
- Bootloader lock/unlock status
- Partition operations (erase, format)
- Flash operations
- Boot operations
- Reboot to different modes

### 🚧 Future Features:
- File browser for image selection
- Progress indication for long operations
- Batch operations
- Firmware validation
- Device-specific profiles
