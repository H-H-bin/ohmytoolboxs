use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/");
    
    // Get build information
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let opt_level = env::var("OPT_LEVEL").unwrap_or_else(|_| "0".to_string());
    
    println!("cargo:rustc-env=TARGET={}", target);
    println!("cargo:rustc-env=PROFILE={}", profile);
    println!("cargo:rustc-env=OPT_LEVEL={}", opt_level);
    
    // Set version info from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    let name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "ohmytoolboxs".to_string());
    let description = env::var("CARGO_PKG_DESCRIPTION").unwrap_or_else(|_| "Desktop Toolbox Application".to_string());
    
    println!("cargo:rustc-env=APP_VERSION={}", version);
    println!("cargo:rustc-env=APP_NAME={}", name);
    println!("cargo:rustc-env=APP_DESCRIPTION={}", description);
    
    // Get git information if available
    if let Ok(git_hash) = get_git_hash() {
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    } else {
        println!("cargo:rustc-env=GIT_HASH=unknown");
    }
    
    if let Ok(git_branch) = get_git_branch() {
        println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
    } else {
        println!("cargo:rustc-env=GIT_BRANCH=unknown");
    }
    
    // Get build timestamp
    let build_timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_timestamp);
    
    // Create dist directory for release builds
    if profile == "release" {
        create_dist_directory();
    }
    
    // Platform-specific configurations
    configure_platform_specific();
    
    println!("Build configuration completed successfully!");
}

fn get_git_hash() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err("Git command failed".into())
    }
}

fn get_git_branch() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err("Git command failed".into())
    }
}

fn create_dist_directory() {
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        if let Err(e) = fs::create_dir_all(dist_dir) {
            println!("cargo:warning=Failed to create dist directory: {}", e);
        } else {
            println!("cargo:warning=Created dist directory for release artifacts");
        }
    }
    
    // Create README for dist directory
    let readme_content = format!(
        "# OhMyToolboxs Distribution\n\n\
        This directory contains the built application artifacts.\n\n\
        ## Files\n\
        - `ohmytoolboxs` or `ohmytoolboxs.exe` - Main application executable\n\
        - Any additional resources or dependencies\n\n\
        ## Build Information\n\
        - Version: {}\n\
        - Built on: {}\n\
        - Target: {}\n",
        env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string()),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        env::var("TARGET").unwrap_or_else(|_| "unknown".to_string())
    );
    
    let readme_path = dist_dir.join("README.md");
    if let Err(e) = fs::write(readme_path, readme_content) {
        println!("cargo:warning=Failed to create dist README: {}", e);
    }
}

fn configure_platform_specific() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    match target_os.as_str() {
        "windows" => {
            // Windows-specific configuration
            // Note: Don't set /SUBSYSTEM:WINDOWS here as it conflicts with main() function
            // Use #![windows_subsystem = "windows"] in main.rs if you want a GUI-only app
            
            // Embed application manifest for Windows
            if Path::new("app.manifest").exists() {
                println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
                println!("cargo:rustc-link-arg=/MANIFESTINPUT:app.manifest");
            }
            
            // Set Windows version info
            set_windows_version_info();
        },
        "macos" => {
            // macOS-specific configuration
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        },
        "linux" => {
            // Linux-specific configuration
            println!("cargo:rustc-link-lib=X11");
            println!("cargo:rustc-link-lib=Xi");
            println!("cargo:rustc-link-lib=Xrandr");
        },
        _ => {
            println!("cargo:warning=Unknown target OS: {}", target_os);
        }
    }
}

#[cfg(windows)]
fn set_windows_version_info() {
    // This would typically use a Windows resource file (.rc)
    // For now, we'll just set some basic flags
    println!("cargo:rustc-link-arg=/VERSION:0.1");
}

#[cfg(not(windows))]
fn set_windows_version_info() {
    // No-op for non-Windows platforms
}
