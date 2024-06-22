use std::fs;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Renderer {
    DX8,
    DX9,
    DX10,
    DX11,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShadowMapSize {
    Size1536,
    Size2048,
    Size2560,
    Size3072,
    Size4096,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AppConfig {
    pub renderer: Renderer,
    pub use_avx: bool,
    pub shadow_map: ShadowMapSize,
    pub debug: bool,
    pub prefetch_sounds: bool,
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
    pub fn load() -> Self {
        let file_data = fs::read_to_string("launcherconfig.toml").unwrap();
        toml::from_str::<AppConfig>(&file_data).unwrap()
    }

    pub fn write(&self) {
        let string_config = toml::to_string(self).unwrap();
        fs::write("launcherconfig.toml", string_config).unwrap();
    }
}
