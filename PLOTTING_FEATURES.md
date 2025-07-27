# Performance Plotting Features

This document describes the interactive plotting capabilities added to the ADB Device Monitor in OhMyToolboxs.

## 📊 Overview

The plotting features provide real-time visualization of Android device performance metrics, allowing users to monitor trends and identify performance patterns over time. The plots are powered by `egui_plot` and offer smooth, interactive visualization.

## 🎯 Key Features

### Real-time Data Visualization
- **Live Updates**: Plots update automatically during monitoring sessions
- **Time-Series Data**: X-axis represents elapsed time since monitoring started
- **Smooth Animation**: Seamless data point addition with configurable intervals
- **Performance Optimized**: Configurable data point limit (default: 1000 points) for optimal rendering
- **Data Point Management**: Automatic removal of oldest data when limit is reached

### Interactive Plot Controls
- **Toggle Plots**: Show/hide plot visualization with checkbox control
- **Zoom and Pan**: Standard egui_plot interaction for detailed analysis
- **Color Coding**: Each metric has a distinct color for easy identification
- **Data Management**: Clear plot history while maintaining monitoring session
- **Configurable Buffer**: Adjust maximum data points (10-10,000 range) for memory and performance tuning

## 📈 Available Plots

### 1. CPU Load Average Plot
- **Color**: Red (`rgb(255, 100, 100)`)
- **Data Source**: `/proc/loadavg` (1-minute load average)
- **Y-Axis**: CPU load value
- **Interpretation**: Higher values indicate increased system load

### 2. Memory Usage Plot
- **Color**: Green (`rgb(100, 255, 100)`)
- **Data Source**: `/proc/meminfo` (calculated percentage)
- **Y-Axis**: Memory usage percentage (0-100%)
- **Calculation**: `(Total - Available) / Total * 100`

### 3. Battery Level Plot
- **Color**: Blue (`rgb(100, 100, 255)`)
- **Data Source**: `dumpsys battery` (level field)
- **Y-Axis**: Battery charge percentage (0-100%)
- **Updates**: Real-time battery level changes

### 4. Battery Temperature Plot
- **Color**: Yellow (`rgb(255, 255, 100)`)
- **Data Source**: `dumpsys battery` (temperature field)
- **Y-Axis**: Temperature in Celsius
- **Conversion**: Automatic conversion from millidegrees to Celsius

## 🔧 Technical Implementation

### Data Collection
```rust
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
    pub max_points: usize, // Default: 100
}
```

### Data Parsing Functions
- **`parse_cpu_load()`**: Extracts 1-minute load average from loadavg output
- **`parse_memory_usage()`**: Calculates memory usage percentage
- **`parse_battery_level()`**: Extracts battery percentage from dumpsys
- **`parse_battery_temperature()`**: Converts temperature to Celsius

### Plot Rendering
```rust
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
```

## 🎛️ User Interface

### Plot Controls
- **📊 Show Plots**: Checkbox to toggle plot visibility
- **Max Data Points**: Drag control to set buffer size (10-10,000 points, default: 1000)
- **🗑️ Clear Plot Data**: Button to reset all plot history
- **Data Points Counter**: Display current/maximum number of data points (e.g., "450 / 1000")

### Plot Layout
- **Height**: 150 pixels per plot for optimal visibility
- **Aspect Ratio**: 3:1 for time-series visualization
- **Spacing**: Proper separation between different metric plots
- **Labels**: Clear plot titles and metric names

### Integration with Monitoring
- **Synchronized Updates**: Plots update with the same interval as monitoring data
- **State Persistence**: Plot preferences saved with application state
- **Memory Management**: Automatic data point cleanup to prevent memory growth

## 🚀 Usage Instructions

### Basic Usage
1. **Connect Device**: Ensure Android device is connected via ADB
2. **Start Monitoring**: Click "▶️ Start Monitoring" in Device Monitor section
3. **Enable Plots**: Check "📊 Show Plots" checkbox
4. **View Trends**: Watch real-time performance visualization
5. **Interact**: Zoom and pan plots for detailed analysis

### Advanced Features
- **Data Analysis**: Use zoom to examine specific time periods
- **Performance Comparison**: Compare multiple metrics simultaneously
- **Data Export**: Plot data is stored in serializable format
- **Troubleshooting**: Clear plot data if visualization becomes cluttered

## 📊 Data Interpretation

### CPU Load Average
- **< 1.0**: System is underutilized
- **1.0**: System is fully utilized but not overloaded
- **> 1.0**: System is overloaded, tasks are queuing

### Memory Usage
- **< 60%**: Good memory availability
- **60-80%**: Moderate memory usage
- **> 80%**: High memory usage, potential performance impact

### Battery Level
- **Decreasing Trend**: Normal battery drain during usage
- **Flat Line**: Device is charging or in low-power mode
- **Sharp Drops**: High power consumption events

### Battery Temperature
- **< 35°C**: Normal operating temperature
- **35-40°C**: Warm but acceptable
- **> 40°C**: Hot, may indicate thermal throttling

## ⚠️ Performance Considerations

### Data Management
- **Max Points**: User-configurable limit (10-10,000 data points per metric, default: 1000)
- **Dynamic Adjustment**: Real-time modification of buffer size with automatic trimming
- **Memory Usage**: Automatic cleanup prevents memory leaks
- **Update Frequency**: Configurable intervals (1-10 seconds)

### UI Performance
- **Efficient Rendering**: Only visible plots are rendered
- **Smooth Updates**: Non-blocking data collection
- **Responsive Interface**: Plot interactions don't block monitoring

### System Impact
- **Minimal Overhead**: Lightweight data collection
- **ADB Efficiency**: Batched commands reduce communication overhead
- **Resource Management**: Proper cleanup on monitoring stop

## 🔍 Troubleshooting

### No Plot Data
- Verify monitoring is enabled and running
- Check that device is properly connected
- Ensure ADB commands are executing successfully

### Performance Issues
- Reduce monitoring interval to decrease update frequency
- Clear plot data to reset visualization
- Check system resources on host computer

### Missing Metrics
- Some data may require specific Android permissions
- Battery data availability varies by device
- Thermal sensors may not be available on all devices

The plotting system provides comprehensive insight into Android device performance trends, making it an invaluable tool for developers, testers, and system administrators who need to monitor device behavior over time.
