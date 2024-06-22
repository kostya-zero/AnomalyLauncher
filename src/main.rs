#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
    process::exit, sync::Arc,
};

mod app_config;
mod game;
mod styles;

use app_config::{AppConfig, Renderer, ShadowMapSize};
use eframe::egui::{
    self, vec2, Button, ComboBox, FontData, FontDefinitions, FontFamily, IconData, RichText, Stroke, Vec2, ViewportBuilder
};
use game::Game;
use rfd::MessageDialog;
use styles::Styles;

fn main() -> eframe::Result<()> {
    if !Path::new("launcherconfig.toml").exists() {
        let default_config = AppConfig::default();
        default_config.write();
    }

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "OpenSans".to_owned(),
        FontData::from_static(include_bytes!("../assets/open_sans.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "OpenSans".to_owned());

    let icon_data = include_bytes!("../assets/icon.ico");

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(icon_data)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
    let arc_icon = Arc::new(IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    });

    let viewport = ViewportBuilder::default()
        .with_maximize_button(false)
        .with_resizable(false)
        .with_inner_size(Vec2 { x: 510.0, y: 195.0 })
        .with_icon(arc_icon);

    eframe::run_native(
        "Anomaly Launcher v1.0.0",
        eframe::NativeOptions {
            viewport,
            vsync: false,
            ..Default::default()
        },
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Box::new(LauncherApp::new(cc))
        }),
    )
}

#[derive(Debug)]
struct LauncherApp {
    config: AppConfig,
    app_shutdown: bool,
}

impl LauncherApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        LauncherApp {
            config,
            app_shutdown: false,
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
            ShadowMapSize::Size1536 => write!(f, "Shadow Map 1536"),
            ShadowMapSize::Size2048 => write!(f, "Shadow Map 2048"),
            ShadowMapSize::Size2560 => write!(f, "Shadow Map 2560"),
            ShadowMapSize::Size3072 => write!(f, "Shadow Map 3072"),
            ShadowMapSize::Size4096 => write!(f, "Shadow Map 4096"),
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.visuals().dark_mode {
                ui.style_mut().visuals = Styles::dark();
            } else {
                ui.style_mut().visuals = Styles::light();
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.vertical(|ui| {
                        ui.style_mut().spacing.item_spacing = vec2(0., 0.);
                        ui.label(RichText::new("S.T.A.L.K.E.R Anomaly").size(24.0));
                        ui.horizontal(|ui| {
                            ui.label("For S.T.A.L.K.E.R Anomaly 1.5.1 and above.");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.set_min_size(vec2(220., 100.));
                        ui.vertical(|ui| {
                            ui.set_min_size(vec2(150., 100.));
                            ui.label(RichText::new("Graphics"));
                            ui.radio_value(&mut self.config.renderer, Renderer::DX8, "DirectX 8");
                            ui.radio_value(&mut self.config.renderer, Renderer::DX9, "DirectX 9");
                            ui.radio_value(&mut self.config.renderer, Renderer::DX10, "DirectX 10");
                            ui.radio_value(&mut self.config.renderer, Renderer::DX11, "DirectX 11");
                            ComboBox::from_id_source("shadow_map")
                                .selected_text(self.config.shadow_map.to_string())
                                .width(150.)
                                .show_ui(ui, |ui| {
                                    ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size1536, "Shadow Map 1536");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size2048, "Shadow Map 2048");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size2560, "Shadow Map 2560");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size3072, "Shadow Map 3072");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size4096, "Shadow Map 4096");
                                });
                        });
                        ui.vertical(|ui| {
                            ui.set_min_size(vec2(150., 100.));
                            ui.label(RichText::new("Misc settings"));
                            ui.checkbox(&mut self.config.debug, "Debug Mode");
                            ui.checkbox(&mut self.config.prefetch_sounds, "Prefetch Sounds");
                            ui.checkbox(&mut self.config.use_avx, "Use AVX");
                        });
                    });
                });
                ui.vertical(|ui| {
                    let play_button = ui.add_sized([180., 65.], Button::new("Play"));
                    let clear_button = ui.add_sized([180., 35.], Button::new("Clear Shader Cache"));
                    let about_button = ui.add_sized([180., 35.], Button::new("About Launcher"));
                    let quit_button = ui.add_sized([180., 35.], Button::new("Quit"));
                    if play_button.clicked() {
                        println!("{:?}", self);
                        let game = Game::new(self.config.renderer, self.config.use_avx);
                        let mut args: Vec<String> = Vec::new();
                        let shadows_arg: String = match self.config.shadow_map {
                            ShadowMapSize::Size1536 => "-smap1536".to_string(),
                            ShadowMapSize::Size2048 => "-smap2048".to_string(),
                            ShadowMapSize::Size2560 => "-smap2560".to_string(),
                            ShadowMapSize::Size3072 => "-smap3072".to_string(),
                            ShadowMapSize::Size4096 => "-smap4096".to_string(),
                        };
                        args.push(shadows_arg);
                        if self.config.debug {
                            args.push("-dbg".to_string());
                        }

                        if self.config.prefetch_sounds {
                            args.push("-prefetch_sounds".to_string());
                        }
                        let launch_result = game.launch(args);
                        if let Err(e) = launch_result {
                            match e {
                                game::GameError::ExecutableNotFound => {
                                    MessageDialog::new()
                                        .set_title("Executable not found")
                                        .set_description("Could not find the executable file of the game. Make sure you run the launcher from the game folder.")
                                        .set_level(rfd::MessageLevel::Error)
                                        .set_buttons(rfd::MessageButtons::Ok)
                                        .show();
                                },
                                game::GameError::Unknown(i) => {
                                    MessageDialog::new()
                                        .set_title("Unknown error occured")
                                        .set_description(format!("The launcher failed to launch the game due to an unexpected error: {}",i))
                                        .set_level(rfd::MessageLevel::Error)
                                        .set_buttons(rfd::MessageButtons::Ok)
                                        .show();
                                },
                            }
                        } else {
                            self.app_shutdown = true;
                        }
                    }

                    if clear_button.clicked() {
                        let mut cache_path: PathBuf = env::current_dir().unwrap();
                        cache_path.push("appdata\\shaders_cache");
                        println!("{:?}", cache_path);
                        if !cache_path.exists() {
                            let _ = MessageDialog::new()
                            .set_title("Path not found")
                            .set_description("The launcher cannot find the shader cache folder. Make sure you run the launcher in the Anomaly game folder.")
                            .set_level(rfd::MessageLevel::Error)
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                        } else {
                            fs::remove_dir_all(cache_path.clone()).unwrap();
                            fs::create_dir(cache_path.clone()).unwrap();
                            MessageDialog::new()
                            .set_title("Clear Shader Cache")
                            .set_description("Shader cache has been deleted.")
                            .set_level(rfd::MessageLevel::Info)
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                        }
                    }

                    if about_button.clicked() {
                        MessageDialog::new()
                        .set_title("About Launcher")
                        .set_buttons(rfd::MessageButtons::Ok)
                        .set_level(rfd::MessageLevel::Info)
                        .set_description(r#"Anomaly Launcher for S.T.A.L.K.E.R Anomaly 1.5.1 and above.

Made by Konstantin "ZERO" Zhigaylo (@kostya_zero). 
This software has open source code on GitHub.

https://github.com/kostya-zero/AnomalyLauncher"#).show();
                    }

                    if quit_button.clicked() {
                        self.app_shutdown = true;
                    }
                });
            });
        });

        // Handle close via close button
        if ctx.input(|i| i.viewport().close_requested()) {
            self.app_shutdown = true;
        }

        if self.app_shutdown {
            self.config.write();
            exit(0);
        }
    }
}
