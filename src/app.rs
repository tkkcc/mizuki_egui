use std::collections::BTreeMap;

use egui::{DragValue, FontData, FontDefinitions, FontFamily, FontId, Style, TextEdit, TextStyle};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    value: f32,
    checked: bool,
    text: String,
    fight: String,
    max_drug: String,
    max_stone: String,
    auto_recruit0: bool,
    auto_recruit1: bool,
    auto_recruit4: bool,
    auto_recruit5: bool,
    auto_recruit6: bool,
}
//
// impl Default for TemplateApp {
//     fn default() -> Self {
//         Self {
//             // Example stuff:
//             label: "Hello World!".to_owned(),
//             value: 0.1,
//             checked: false,
//             text: String::from("ok"),
//             fight: String::from("jm hd ce ls ap pr"),
//         }
//     }
// }

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "zpix".to_owned(),
        FontData::from_static(include_bytes!("../zpix.ttf")),
        // egui::FontData::from_static(include_bytes!("/home/bilabila/.local/share/fonts/simhei.ttf")),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "zpix".to_owned());

    ctx.set_fonts(fonts);

    /// The default text styles of the default egui theme.
    fn default_text_styles() -> BTreeMap<TextStyle, FontId> {
        use FontFamily::{Monospace, Proportional};

        [
            (TextStyle::Small, FontId::new(18.0, Proportional)),
            (TextStyle::Body, FontId::new(25.0, Proportional)),
            (TextStyle::Button, FontId::new(25.0, Proportional)),
            (TextStyle::Heading, FontId::new(36.0, Proportional)),
            (TextStyle::Monospace, FontId::new(24.0, Monospace)),
        ]
        .into()
    }

    let style = Style {
        text_styles: default_text_styles(),
        ..Style::default()
    };

    // ctx.set_style(style)
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        // cc.egui_ctx.set_fonts(font_definitions);
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        // cc.egui_ctx.set_pixels_per_point(2.0);
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            checked,
            text,
            fight,
            max_drug,
            max_stone,
            auto_recruit0,
            auto_recruit1,
            auto_recruit4,
            auto_recruit5,
            auto_recruit6,
        } = self;

        egui::TopBottomPanel::bottom("my_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("dd");
            });
        });

        let control = |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.centered(|ui| {
                    ui.label("明日方舟速通 611-12.01");
                });
                ui.horizontal(|ui| {
                    ui.label("作战关卡");
                    let txt = TextEdit::multiline(fight).desired_rows(1);
                    ui.add(txt);
                });
                ui.horizontal(|ui| {
                    ui.label("作战吃药");
                    let txt = TextEdit::singleline(max_stone)
                        .desired_rows(1)
                        .desired_width(20.0);
                    ui.add(txt);
                    ui.label("次，吃石头");
                    let txt = TextEdit::singleline(max_drug)
                        .desired_rows(1)
                        .desired_width(20.0);
                    ui.add(txt);
                    ui.label("次");
                });

                ui.horizontal_top(|ui| {
                    ui.label("信用多买");
                    let txt = TextEdit::multiline(fight)
                        .desired_rows(1)
                        .desired_width(75.0);
                    ui.add(txt);
                    ui.label("信用少买");
                    let txt = TextEdit::multiline(fight)
                        .desired_rows(1)
                        .desired_width(75.0);
                    ui.add(txt);
                });
                ui.horizontal(|ui| {
                    ui.label("自动招募");
                    ui.checkbox(auto_recruit0, "其他");
                    ui.checkbox(auto_recruit1, "车");
                    ui.checkbox(auto_recruit4, "4");
                    ui.checkbox(auto_recruit5, "5");
                    ui.checkbox(auto_recruit6, "6");
                });

                ui.horizontal(|ui| {
                    ui.label("任务列表");

                    egui::Grid::new("job").show(ui, |ui| {
                        ui.checkbox(auto_recruit0, "邮件收取");
                        ui.checkbox(auto_recruit0, "轮次作战");
                        ui.checkbox(auto_recruit0, "访问好友");
                        ui.end_row();
                        ui.checkbox(auto_recruit0, "邮件收取");
                        ui.checkbox(auto_recruit0, "轮次作战");
                        ui.checkbox(auto_recruit0, "访问好友");
                        ui.end_row();
                        ui.checkbox(auto_recruit0, "邮件收取");
                        ui.checkbox(auto_recruit0, "轮次作战");
                        ui.checkbox(auto_recruit0, "限时活动");
                        ui.end_row();
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("完成之后");
                    ui.checkbox(auto_recruit0, "返回桌面");
                    ui.checkbox(auto_recruit1, "关闭游戏");
                    ui.checkbox(auto_recruit4, "熄屏");
                });
                ui.horizontal(|ui| {
                    ui.label("定时启动");
                    let txt = TextEdit::multiline(fight).desired_rows(1);
                    ui.add(txt)
                });
                ui.menu_button("My menu", |ui| {
                    ui.menu_button("My sub-menu", |ui| {
                        if ui.button("Close the menu").clicked() {
                            ui.close_menu();
                        }
                    });
                });
            })
            .response
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.add_sized(egui::vec2(300.0, 0.0), control));
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
