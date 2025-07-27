pub mod adb_tools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ToolCategory {
    AdbTools,
}

impl ToolCategory {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AdbTools,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::AdbTools => "ADB Tools",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::AdbTools => "🤖",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::AdbTools => "Android Debug Bridge utilities",
        }
    }
}
