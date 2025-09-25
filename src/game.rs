use anyhow::{anyhow, Result};
use std::{env, process::Command};

use crate::Renderer;

pub fn launch_game(dx_level: Renderer, use_avx: bool, args: Vec<String>) -> Result<()> {
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
    if !cwd.exists() {
        return Err(anyhow!("Executable not found"));
    }

    let mut cmd = Command::new(cwd);

    cmd.args(args);
    let result = cmd.spawn();
    if let Err(err) = result {
        return Err(anyhow!("Unknown error occurred: {}", err.to_string()));
    }

    Ok(())
}
