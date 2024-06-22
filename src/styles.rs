use eframe::egui::{style::{WidgetVisuals, Widgets}, Color32, Stroke, Visuals};

pub struct Styles;
impl Styles {
    pub fn light() -> Visuals {
        Visuals {
            dark_mode: false,
            override_text_color: Some(Color32::from_rgb(25, 25, 25)),
            widgets: Widgets {
                hovered: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::light().widgets.hovered
                },
                active: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::light().widgets.hovered
                },
                ..Visuals::light().widgets
            },
            ..Visuals::light()
        }
    }

    pub fn dark() -> Visuals {
        Visuals {
            dark_mode: true,
            override_text_color: Some(Color32::from_rgb(225, 225, 225)),
            widgets: Widgets {
                hovered: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::dark().widgets.hovered
                },
                active: WidgetVisuals {
                    bg_stroke: Stroke::NONE,
                    ..Visuals::dark().widgets.hovered
                },
                ..Visuals::dark().widgets
            },
            ..Visuals::dark()
        }
    }
}
