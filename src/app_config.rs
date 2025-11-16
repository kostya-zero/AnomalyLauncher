use serde::{Deserialize, Serialize};
use std::{fmt, fs};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Renderer {
    DX8,
    DX9,
    DX10,
    DX11,
}

impl From<Renderer> for String {
    fn from(val: Renderer) -> Self {
        match val {
            Renderer::DX8 => "DirectX 8".to_string(),
            Renderer::DX9 => "DirectX 9".to_string(),
            Renderer::DX10 => "DirectX 10".to_string(),
            Renderer::DX11 => "DirectX 11".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShadowMapSize {
    Size1536,
    Size2048,
    Size2560,
    Size3072,
    Size4096,
}

impl From<ShadowMapSize> for String {
    fn from(val: ShadowMapSize) -> Self {
        match val {
            ShadowMapSize::Size1536 => "1536".to_string(),
            ShadowMapSize::Size2048 => "2048".to_string(),
            ShadowMapSize::Size2560 => "2560".to_string(),
            ShadowMapSize::Size3072 => "3072".to_string(),
            ShadowMapSize::Size4096 => "4096".to_string(),
        }
    }
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Renderer::DX8 => write!(f, "DirectX 8"),
            Renderer::DX9 => write!(f, "DirectX 9"),
            Renderer::DX10 => write!(f, "DirectX 10"),
            Renderer::DX11 => write!(f, "DirectX 11"),
        }
    }
}

impl fmt::Display for ShadowMapSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShadowMapSize::Size1536 => write!(f, "1536"),
            ShadowMapSize::Size2048 => write!(f, "2048"),
            ShadowMapSize::Size2560 => write!(f, "2560"),
            ShadowMapSize::Size3072 => write!(f, "3072"),
            ShadowMapSize::Size4096 => write!(f, "4096"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AppConfig {
    pub renderer: Renderer,
    pub use_avx: bool,
    pub shadow_map: ShadowMapSize,
    pub debug: bool,
    pub prefetch_sounds: bool,
}

pub enum AppConfigError {
    ReadFailed,
    BadStructure,
    WriteFailed,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            renderer: Renderer::DX10,
            shadow_map: ShadowMapSize::Size2048,
            debug: true,
            use_avx: true,
            prefetch_sounds: false,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, AppConfigError> {
        let content =
            fs::read_to_string("launcherconfig.toml").map_err(|_| AppConfigError::ReadFailed)?;
        toml::from_str(&content).map_err(|_| AppConfigError::BadStructure)
    }

    pub fn write(&self) -> Result<(), AppConfigError> {
        let string_config = toml::to_string(self).unwrap();
        fs::write("launcherconfig.toml", string_config).map_err(|_| AppConfigError::WriteFailed)
    }
}
