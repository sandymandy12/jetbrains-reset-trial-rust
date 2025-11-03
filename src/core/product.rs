use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JetBrainsProduct {
    IntelliJIdea,
    PyCharm,
    WebStorm,
    PhpStorm,
    CLion,
    GoLand,
    Rider,
    DataGrip,
    RubyMine,
    RustRover,
    AndroidStudio,
    Fleet,
}

impl JetBrainsProduct {
    pub fn from_dir_name(name: &str) -> Option<Self> {
        if name.starts_with("IntelliJIdea") {
            Some(Self::IntelliJIdea)
        } else if name.starts_with("PyCharm") {
            Some(Self::PyCharm)
        } else if name.starts_with("WebStorm") {
            Some(Self::WebStorm)
        } else if name.starts_with("PhpStorm") {
            Some(Self::PhpStorm)
        } else if name.starts_with("CLion") {
            Some(Self::CLion)
        } else if name.starts_with("GoLand") {
            Some(Self::GoLand)
        } else if name.starts_with("Rider") {
            Some(Self::Rider)
        } else if name.starts_with("DataGrip") {
            Some(Self::DataGrip)
        } else if name.starts_with("RubyMine") {
            Some(Self::RubyMine)
        } else if name.starts_with("RustRover") {
            Some(Self::RustRover)
        } else if name.starts_with("AndroidStudio") {
            Some(Self::AndroidStudio)
        } else if name.starts_with("Fleet") {
            Some(Self::Fleet)
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::IntelliJIdea => "IntelliJ IDEA",
            Self::PyCharm => "PyCharm",
            Self::WebStorm => "WebStorm",
            Self::PhpStorm => "PhpStorm",
            Self::CLion => "CLion",
            Self::GoLand => "GoLand",
            Self::Rider => "Rider",
            Self::DataGrip => "DataGrip",
            Self::RubyMine => "RubyMine",
            Self::RustRover => "RustRover",
            Self::AndroidStudio => "Android Studio",
            Self::Fleet => "Fleet",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::IntelliJIdea => "🧠",
            Self::PyCharm => "🐍",
            Self::WebStorm => "🌐",
            Self::PhpStorm => "🐘",
            Self::CLion => "⚙️",
            Self::GoLand => "🐹",
            Self::Rider => "🎮",
            Self::DataGrip => "🗄️",
            Self::RubyMine => "💎",
            Self::RustRover => "🦀",
            Self::AndroidStudio => "🤖",
            Self::Fleet => "🚀",
        }
    }
}

impl fmt::Display for JetBrainsProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductInstall {
    pub product: JetBrainsProduct,
    pub version: String,
    pub config_path: PathBuf,
    pub data_path: PathBuf,
    pub trial_status: TrialStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrialStatus {
    Active { days_remaining: u32 },
    Expired,
    NotStarted,
    Licensed,
    Unknown,
}

impl fmt::Display for TrialStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active { days_remaining } => write!(f, "Active ({} days)", days_remaining),
            Self::Expired => write!(f, "Expired"),
            Self::NotStarted => write!(f, "Not Started"),
            Self::Licensed => write!(f, "Licensed"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResetReport {
    pub product: ProductInstall,
    pub success: bool,
    pub backup_created: bool,
    pub backup_path: Option<PathBuf>,
    pub files_modified: Vec<PathBuf>,
    pub dirs_removed: Vec<PathBuf>,
    pub error: Option<String>,
}

impl ResetReport {
    pub fn new(product: ProductInstall) -> Self {
        Self {
            product,
            success: false,
            backup_created: false,
            backup_path: None,
            files_modified: Vec::new(),
            dirs_removed: Vec::new(),
            error: None,
        }
    }
}
