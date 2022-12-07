use egui::{FontData, FontDefinitions, FontFamily, TextEdit};

enum Mode {
    Daily,
    Roguelike,
    Setting,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    fight: String,
    max_drug: String,
    max_stone: String,
    auto_recruit0: bool,
    auto_recruit1: bool,
    auto_recruit4: bool,
    auto_recruit5: bool,
    auto_recruit6: bool,
    date: String,
    setting_layout: bool
}

pub fn set_style(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "zpix".to_owned(),
        FontData::from_static(include_bytes!("../zpix.ttf")),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "zpix".to_owned());

    ctx.set_fonts(fonts);

    // /// The default text styles of the default egui theme.
    // use std::collections::BTreeMap;
    // fn default_text_styles() -> BTreeMap<TextStyle, FontId> {
    //     use FontFamily::{Monospace, Proportional};
    //
    //     [
    //         (TextStyle::Small, FontId::new(18.0, Proportional)),
    //         (TextStyle::Body, FontId::new(25.0, Proportional)),
    //         (TextStyle::Button, FontId::new(25.0, Proportional)),
    //         (TextStyle::Heading, FontId::new(36.0, Proportional)),
    //         (TextStyle::Monospace, FontId::new(24.0, Monospace)),
    //     ]
    //     .into()
    // }

    let mut style = (*ctx.style()).clone();
    style.animation_time = 0.0;
    ctx.set_style(style)
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_style(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("my_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {


                // let txt = RichText::new("Large and underlined").size(f32::INFINITY);
                ui.label("启用账号");
                if ui.button("启动").clicked() {}
            });
        });

        let control = |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("明日方舟速通 611-12.01");
                });
                ui.horizontal(|ui| {
                    ui.label("作战关卡");
                    let txt = TextEdit::multiline(&mut self.fight).desired_rows(1);
                    ui.add(txt);
                });
                ui.horizontal(|ui| {
                    ui.label("作战吃药");
                    let txt = TextEdit::singleline(&mut self.max_stone)
                        .desired_rows(1)
                        .desired_width(20.0);
                    ui.add(txt);
                    ui.label("次，吃石头");
                    let txt = TextEdit::singleline(&mut self.max_drug)
                        .desired_rows(1)
                        .desired_width(20.0);
                    ui.add(txt);
                    ui.label("次");
                });

                ui.horizontal_top(|ui| {
                    ui.label("信用多买");
                    let txt = TextEdit::multiline(&mut self.fight)
                        .desired_rows(1)
                        .desired_width(75.0);
                    ui.add(txt);
                    ui.label("信用少买");
                    let txt = TextEdit::multiline(&mut self.fight)
                        .desired_rows(1)
                        .desired_width(75.0);
                    ui.add(txt);
                });

                ui.horizontal(|ui| {
                    ui.label("自动招募");
                    ui.checkbox(&mut self.auto_recruit0, "其他");
                    ui.checkbox(&mut self.auto_recruit0, "车");
                    ui.checkbox(&mut self.auto_recruit0, "4");
                    ui.checkbox(&mut self.auto_recruit0, "5");
                    ui.checkbox(&mut self.auto_recruit6, "6");
                });

                ui.horizontal(|ui| {
                    ui.label("任务列表");

                    egui::Grid::new("job").show(ui, |ui| {
                        ui.checkbox(&mut self.auto_recruit0, "邮件收取");
                        ui.checkbox(&mut self.auto_recruit0, "轮次作战");
                        ui.checkbox(&mut self.auto_recruit0, "访问好友");
                        ui.end_row();
                        ui.checkbox(&mut self.auto_recruit0, "邮件收取");
                        ui.checkbox(&mut self.auto_recruit0, "轮次作战");
                        ui.checkbox(&mut self.auto_recruit0, "访问好友");
                        ui.end_row();
                        ui.checkbox(&mut self.auto_recruit0, "邮件收取");
                        ui.checkbox(&mut self.auto_recruit0, "轮次作战");
                        ui.checkbox(&mut self.auto_recruit0, "限时活动");
                        ui.end_row();
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("允许时间");
                    egui::Grid::new("allow_week").show(ui, |ui| {
                        ui.checkbox(&mut self.auto_recruit0, "周一");
                        ui.checkbox(&mut self.auto_recruit0, "周二");
                        ui.checkbox(&mut self.auto_recruit0, "周三");
                        ui.checkbox(&mut self.auto_recruit0, "周四");
                        ui.end_row();
                        ui.checkbox(&mut self.auto_recruit0, "周五");
                        ui.checkbox(&mut self.auto_recruit0, "周六");
                        ui.checkbox(&mut self.auto_recruit0, "周日");
                    });
                });

                ui.horizontal(|ui| {
                    ui.label("完成之后");
                    ui.checkbox(&mut self.auto_recruit0, "返回桌面");
                    ui.checkbox(&mut self.auto_recruit0, "关闭游戏");
                    ui.checkbox(&mut self.auto_recruit0, "熄屏");
                });
                ui.horizontal(|ui| {
                    ui.label("定时启动");
                    let txt = TextEdit::multiline(&mut self.fight).desired_rows(1);
                    ui.add(txt)
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
