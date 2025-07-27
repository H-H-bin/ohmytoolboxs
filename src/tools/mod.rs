pub mod adb_tools;
pub mod fastboot_tools;
pub mod qdl_tools;
pub mod qramdump_tools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ToolCategory {
    AdbTools,
    FastbootTools,
    QdlTools,
    QramdumpTools,
}

impl ToolCategory {    pub fn all() -> Vec<Self> {
        vec![
            Self::AdbTools,
            Self::FastbootTools,
            Self::QdlTools,
            Self::QramdumpTools,
        ]
    }    pub fn name(&self) -> &'static str {
        match self {
            Self::AdbTools => "ADB Tools",
            Self::FastbootTools => "Fastboot Tools",
            Self::QdlTools => "QDL Tools",
            Self::QramdumpTools => "QRamdump Tools",
        }
    }    pub fn icon(&self) -> &'static str {
        match self {
            Self::AdbTools => "🤖",
            Self::FastbootTools => "⚡",
            Self::QdlTools => "📱",
            Self::QramdumpTools => "🧠",
        }
    }    pub fn description(&self) -> &'static str {
        match self {
            Self::AdbTools => "Android Debug Bridge utilities",
            Self::FastbootTools => "Android Fastboot flashing utilities",
            Self::QdlTools => "Qualcomm EDL/9008 mode tools",
            Self::QramdumpTools => "Qualcomm memory dump collection tools",
        }
    }
}
