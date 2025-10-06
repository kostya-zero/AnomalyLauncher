#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::RefCell;
use std::rc::Rc;
use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
    process::exit,
    sync::Arc,
};

use crate::game::launch_game;
use app_config::{AppConfig, Renderer, ShadowMapSize};
use eframe::egui::{
    self, vec2, Align, Button, ComboBox, FontData, FontDefinitions, FontFamily, FontId, IconData,
    Layout, RichText, Stroke, TextStyle, Vec2, ViewportBuilder, ViewportId,
};
use rfd::MessageDialog;
use styles::Styles;

mod app_config;
mod game;
mod styles;

const GEIST_REGULAR: &[u8] = include_bytes!("../assets/geist_regular.otf");
const GEIST_BOLD: &[u8] = include_bytes!("../assets/geist_bold.otf");
static FONT_FAMILY_BOLD: &str = "bold";

pub fn font_family_bold() -> FontFamily {
    FontFamily::Name(Arc::from(FONT_FAMILY_BOLD))
}

pub fn font_id_heading() -> FontId {
    FontId::new(24., font_family_bold())
}

fn show_error(title: &str, desc: &str) {
    MessageDialog::new()
        .set_title(title)
        .set_description(desc)
        .set_level(rfd::MessageLevel::Error)
        .set_buttons(rfd::MessageButtons::Ok)
        .show();
}

fn load_icon_data() -> Result<IconData, image::ImageError> {
    let icon_data = include_bytes!("../assets/icon.ico");
    let image = image::load_from_memory(icon_data)?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Ok(IconData {
        rgba,
        width,
        height,
    })
}

fn load_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "geist".to_string(),
        FontData::from_static(GEIST_REGULAR).into(),
    );

    fonts.font_data.insert(
        "geist-bold".to_string(),
        FontData::from_static(GEIST_BOLD).into(),
    );

    {
        let prop = fonts.families.get_mut(&FontFamily::Proportional).unwrap();
        prop.insert(0, "geist".to_string());
    }

    fonts
        .families
        .insert(font_family_bold(), vec!["geist-bold".to_string()]);

    fonts
}

// Trait to make TRUE BOLD text
trait RichTextExt {
    fn bold(self) -> Self;
}

impl RichTextExt for RichText {
    fn bold(self) -> Self {
        self.family(font_family_bold())
    }
}

fn main() -> eframe::Result<()> {
    if !Path::new("launcherconfig.toml").exists() {
        let default_config = AppConfig::default();
        let _ = default_config.write();
    }

    let icon_data = match load_icon_data() {
        Ok(data) => Arc::new(data),
        Err(_) => {
            show_error("Icon Error", "Failed to load application icon.");
            exit(1);
        }
    };

    let viewport = ViewportBuilder::default()
        .with_maximize_button(false)
        .with_resizable(false)
        .with_inner_size(Vec2 { x: 510.0, y: 195.0 })
        .with_icon(icon_data);

    eframe::run_native(
        "Anomaly Launcher",
        eframe::NativeOptions {
            viewport,
            vsync: false,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(LauncherApp::new(cc)))),
    )
}

#[derive(Debug)]
struct LauncherApp {
    config: AppConfig,
    app_shutdown: bool,
    open_about: Rc<RefCell<bool>>,
}

impl LauncherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load().unwrap_or_else(|err| {
            match err {
                app_config::AppConfigError::ReadFailed => show_error("Read Failed", "Failed to read the configuration file. Please remove 'launcherconfig.toml' and try to launch program again."),
                app_config::AppConfigError::BadStructure => show_error("Bad configuration", "Your configuration seems to be damaged. Please remove 'launcherconfig.toml' and try to launch program again."),
                app_config::AppConfigError::WriteFailed => show_error("Write Failed", "Your configuration seems to be damaged. Please remove 'launcherconfig.toml' and try to launch program again."),
            };
            exit(1);
        });

        // Configuring fonts
        cc.egui_ctx.set_fonts(load_fonts());
        cc.egui_ctx.all_styles_mut(|style| {
            *style.text_styles.get_mut(&TextStyle::Heading).unwrap() = font_id_heading()
        });

        LauncherApp {
            config,
            app_shutdown: false,
            open_about: Rc::new(RefCell::new(false)),
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

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let open = *self.open_about.borrow();
        let mut open_flag = self.open_about.borrow_mut();
        if open {
            let viewport = ViewportBuilder::default()
                .with_title("About this Launcher")
                .with_inner_size([350.0, 230.0])
                .with_resizable(false);

            // FIXME: Replace with `show_viewport_deferred` in the future
            ctx.show_viewport_immediate(
                ViewportId::from_hash_of("about_viewport"),
                viewport,
                |ctx, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.with_layout(Layout::top_down(Align::Center), |ui| {
                            ui.label(
                                RichText::new("Anomaly Launcher")
                                    .bold().size(24.),
                            );

                            ui.label(
                                RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                                    .font(FontId::proportional(14.0))
                                    .weak(),
                            );

                            ui.add_space(20.0);
                            ui.label(
                                RichText::new("Anomaly Launcher for S.T.A.L.K.E.R Anomaly 1.5.1 and above. Made by Konstantin \"ZERO\" Zhigaylo (@kostya_zero). This software has open source code on GitHub.")
                                    .font(FontId::proportional(12.0))
                            );
                            ui.add_space(20.0);
                            ui.separator();
                            ui.add_space(12.0);

                            ui.hyperlink_to(
                                "View on GitHub",
                                "https://github.com/kostya-zero/AnomalyLauncher",
                            );

                            ui.add_space(16.0);
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        *open_flag = false;
                    }
                },
            );
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.visuals().dark_mode {
                ui.style_mut().visuals = Styles::dark();
            } else {
                ui.style_mut().visuals = Styles::light();
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.style_mut().spacing.item_spacing = vec2(0., 38.);

                    ui.vertical(|ui| {
                        ui.style_mut().spacing.item_spacing = vec2(0., 0.);
                        ui.label(RichText::new("Anomaly Launcher").bold().size(24.0));
                            ui.label(RichText::new("Made by @kostya_zero for stalkers.").weak());
                    });


                    ui.horizontal(|ui| {
                        ui.style_mut().spacing.item_spacing = vec2(6., 6.);

                        ui.set_min_size(vec2(220., 100.));
                        ui.vertical(|ui| {
                            ui.set_min_size(vec2(150., 100.));
                            ui.label(RichText::new("Renderer").bold().size(14.));
                            ComboBox::from_id_salt("renderer")
                                .selected_text(self.config.renderer.to_string())
                                .width(150.)
                                .show_ui(ui, |ui| {
                                    ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                                    ui.selectable_value(&mut self.config.renderer, Renderer::DX8, "DirectX 8");
                                    ui.selectable_value(&mut self.config.renderer, Renderer::DX9, "DirectX 9");
                                    ui.selectable_value(&mut self.config.renderer, Renderer::DX10, "DirectX 10");
                                    ui.selectable_value(&mut self.config.renderer, Renderer::DX11, "DirectX 11");
                                });
                            ui.label(RichText::new("Shadow Map Size").bold().size(14.));
                            ComboBox::from_id_salt("shadow_map")
                                .selected_text(self.config.shadow_map.to_string())
                                .width(150.)
                                .show_ui(ui, |ui| {
                                    ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size1536, "1536");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size2048, "2048");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size2560, "2560");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size3072, "3072");
                                    ui.selectable_value(&mut self.config.shadow_map, ShadowMapSize::Size4096, "4096");
                                });
                        });
                        ui.vertical(|ui| {
                            ui.set_min_size(vec2(150., 100.));
                            ui.label(RichText::new("Misc settings").bold().size(14.));
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
                        let launch_result = launch_game(self.config.renderer, self.config.use_avx, args);
                        if let Err(e) = launch_result {
                            show_error("Launch Failed", format!("Failed to launch Anomaly: {e}").as_str());
                        } else {
                            self.app_shutdown = true;
                        }
                    }

                    if clear_button.clicked() {
                        let mut cache_path: PathBuf = env::current_dir().unwrap();
                        cache_path.push("appdata\\shaders_cache");
                        if !cache_path.exists() {
                            show_error("Path not found", "The launcher cannot find the shader cache folder. Make sure you run the launcher in the Anomaly game folder.")
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
                        *open_flag = true;
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
            match self.config.write() {
                Ok(_) => {}
                Err(_) => show_error(
                    "Write Failed",
                    "Failed to write data to configuration file. You might need to set your options again.",
                ),
            };
            exit(0);
        }
    }
}
