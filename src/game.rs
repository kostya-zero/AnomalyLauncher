use std::{env, path::PathBuf, process::Command};

use crate::Renderer;

pub enum GameError {
    ExecutableNotFound,
    Unknown(String),
}

pub struct Game(PathBuf);
impl Game {
    pub fn new(dx_level: Renderer, use_avx: bool) -> Self {
        let mut cwd = env::current_dir().unwrap();
        match (dx_level, use_avx) {
            (Renderer::DX8, true) => cwd.push("bin\\AnomalyDX8.exe"),
            (Renderer::DX8, false) => cwd.push("bin\\AnomalyDX8AVX.exe"),
            (Renderer::DX9, true) => cwd.push("bin\\AnomalyDX9.exe"),
            (Renderer::DX9, false) => cwd.push("bin\\AnomalyDX9AVX.exe"),
            (Renderer::DX10, true) => cwd.push("bin\\AnomalyDX10.exe"),
            (Renderer::DX10, false) => cwd.push("bin\\AnomalyDX10AVX.exe"),
            (Renderer::DX11, true) => cwd.push("bin\\AnomalyDX11.exe"),
            (Renderer::DX11, false) => cwd.push("bin\\AnomalyDX11AVX.exe"),
        };
        Game(cwd)
    }

    pub fn launch(&self, args: Vec<String>) -> Result<(), GameError> {
        let mut cmd = Command::new(self.0.clone());

        if !self.0.exists() {
            return Err(GameError::ExecutableNotFound);
        }

        cmd.args(args);
        let result = cmd.spawn();
        if let Err(err) = result {
            return Err(GameError::Unknown(err.to_string()));
        }

        Ok(())
    }
}
