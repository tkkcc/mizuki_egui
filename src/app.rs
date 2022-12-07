use egui::{DragValue, FontData, FontDefinitions, FontFamily, ScrollArea, TextEdit};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 350.0;

#[derive(Deserialize, Serialize, PartialEq, Default, Clone)]
enum Server {
    #[default]
    Official,
    Bilibili,
}

enum Mode {
    Daily,
    Roguelike,
    Setting,
}

use derivative;
use derivative::Derivative;
#[derive(serde::Deserialize, serde::Serialize, Clone, Derivative)]
#[derivative(Default)]
#[serde(default)]
pub struct User {
    #[derivative(Default(value = "true"))]
    inherit: bool,
    inherit_index: usize,
    username: String,
    password: String,
    server: Server,
    fight: String,
    max_drug: usize,
    max_stone: usize,
    prefer_goods: String,
    dislike_goods: String,
    #[derivative(Default(value = "true"))]
    auto_recruit0: bool,
    #[derivative(Default(value = "true"))]
    auto_recruit1: bool,
    #[derivative(Default(value = "true"))]
    auto_recruit4: bool,
    #[derivative(Default(value = "true"))]
    auto_recruit5: bool,
    #[derivative(Default(value = "true"))]
    auto_recruit6: bool,
    #[derivative(Default(value = "true"))]
    job_mail: bool,
    #[derivative(Default(value = "true"))]
    job_fight: bool,
    #[derivative(Default(value = "true"))]
    job_friend: bool,
    #[derivative(Default(value = "true"))]
    job_gain: bool,
    #[derivative(Default(value = "true"))]
    job_shift: bool,
    #[derivative(Default(value = "true"))]
    job_manu: bool,
    #[derivative(Default(value = "true"))]
    job_clue: bool,
    #[derivative(Default(value = "true"))]
    job_assist: bool,
    #[derivative(Default(value = "true"))]
    job_shop: bool,
    #[derivative(Default(value = "true"))]
    job_recruit: bool,
    #[derivative(Default(value = "true"))]
    job_task: bool,
    #[derivative(Default(value = "true"))]
    job_activity: bool,
    #[derivative(Default(value = "true"))]
    allow_monday: bool,
    #[derivative(Default(value = "true"))]
    allow_tuesday: bool,
    #[derivative(Default(value = "true"))]
    allow_wednesday: bool,
    #[derivative(Default(value = "true"))]
    allow_thursday: bool,
    #[derivative(Default(value = "true"))]
    allow_friday: bool,
    #[derivative(Default(value = "true"))]
    allow_saturday: bool,
    #[derivative(Default(value = "true"))]
    allow_sunday: bool,
}
impl User {
    fn inherit(mut self, inherit: bool) -> Self {
        self.inherit = inherit;
        self
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    user: Vec<User>,
    multi_account: bool,
    roguelike: bool,
    setting_layout: bool,
    end_restart: bool,
    crontab: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut user = vec![User::default().inherit(false)];
        user.extend(vec![User::default(); 10_0000]);

        Self {
            roguelike: false,
            user,
            setting_layout: false,
            end_restart: false,
            multi_account: true,
            crontab: "4:00 12:00".into(),
        }
    }
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

    fn one_user(ui: &mut egui::Ui, state: &mut Self, idx: usize) {
        ui.add_enabled_ui(state.user[idx].job_fight, |ui| {
            ui.horizontal(|ui| {
                ui.label("作战关卡");
                ui.text_edit_singleline(&mut state.user[idx].fight);
            });
            ui.horizontal(|ui| {
                ui.label("作战吃药");
                ui.add(DragValue::new(&mut state.user[idx].max_drug));
                // let txt = TextEdit::singleline(&mut state.user[idx].max_stone)
                //     .desired_rows(1)
                //     .desired_width(50.0);
                // ui.add(txt);
                ui.label("次，吃石头");
                ui.add(DragValue::new(&mut state.user[idx].max_stone));
                // let txt = TextEdit::singleline(&mut state.user[idx].max_drug)
                //     .desired_rows(1)
                //     .desired_width(50.0);
                // ui.add(txt);
                ui.label("次");
            });
        });
        ui.add_enabled_ui(state.user[idx].job_shop, |ui| {
            ui.horizontal(|ui| {
                ui.label("信用多买");
                ui.text_edit_singleline(&mut state.user[idx].prefer_goods);
            });
            ui.horizontal(|ui| {
                ui.label("信用少买");
                ui.text_edit_singleline(&mut state.user[idx].dislike_goods);
            });
        });

        ui.add_enabled_ui(state.user[idx].job_recruit, |ui| {
            ui.horizontal(|ui| {
                ui.label("自动招募");
                ui.checkbox(&mut state.user[idx].auto_recruit0, "其他");
                ui.checkbox(&mut state.user[idx].auto_recruit1, "小车");
                ui.checkbox(&mut state.user[idx].auto_recruit4, "四星");
                ui.checkbox(&mut state.user[idx].auto_recruit5, "五星");
                ui.checkbox(&mut state.user[idx].auto_recruit6, "六星");
            });
        });

        ui.horizontal(|ui| {
            ui.label("任务列表");

            egui::Grid::new(format!("job{idx}")).show(ui, |ui| {
                ui.checkbox(&mut state.user[idx].job_mail, "邮件收取");
                ui.checkbox(&mut state.user[idx].job_fight, "轮次作战");
                ui.checkbox(&mut state.user[idx].job_friend, "访问好友");
                ui.end_row();
                ui.checkbox(&mut state.user[idx].job_gain, "基建收获");
                ui.checkbox(&mut state.user[idx].job_shift, "基建换班");
                ui.checkbox(&mut state.user[idx].job_manu, "制造加速");
                ui.end_row();
                ui.checkbox(&mut state.user[idx].job_clue, "线索交流");
                ui.checkbox(&mut state.user[idx].job_assist, "副手换人");
                ui.checkbox(&mut state.user[idx].job_shop, "信用购买");
                ui.end_row();
                ui.checkbox(&mut state.user[idx].job_recruit, "公开招募");
                ui.checkbox(&mut state.user[idx].job_task, "任务收集");
                ui.checkbox(&mut state.user[idx].job_activity, "限时活动");
                ui.end_row();
            });
        });

        ui.horizontal(|ui| {
            ui.label("允许时间");
            egui::Grid::new(format!("allow_weekday{idx}")).show(ui, |ui| {
                ui.checkbox(&mut state.user[idx].allow_monday, "周一");
                ui.checkbox(&mut state.user[idx].allow_tuesday, "周二");
                ui.checkbox(&mut state.user[idx].allow_wednesday, "周三");
                ui.checkbox(&mut state.user[idx].allow_thursday, "周四");
                ui.checkbox(&mut state.user[idx].allow_friday, "周五");
                ui.end_row();
                ui.checkbox(&mut state.user[idx].allow_saturday, "周六");
                ui.checkbox(&mut state.user[idx].allow_sunday, "周日");
            });
        });
    }

    fn single_user(ui: &mut egui::Ui, state: &mut Self) {
        Self::one_user(ui, state, 0);
        ui.horizontal(|ui| {
            ui.label("定时启动");
            // ui.checkbox(&mut state.auto_recruit0, "");
            // let txt = TextEdit::multiline(&mut state.fight).desired_rows(1);
            ui.text_edit_singleline(&mut state.crontab)
        });
    }

    fn multi_user(ui: &mut egui::Ui, state: &mut Self) {
        let row_height = HEIGHT;
        let table = TableBuilder::new(ui).column(Column::initial(WIDTH));

        table.body(|body| {
            body.rows(row_height, state.user.len(), |row_index, mut row| {
                row.col(|ui| {
                    let idx = row_index;
                    let id = format!("{idx}");
                    ui.label(id);
                    ui.horizontal(|ui| {
                        ui.label(format!("账号"));
                        ui.text_edit_singleline(&mut state.user[idx].username);
                    });
                    ui.horizontal(|ui| {
                        ui.label(format!("密码"));
                        ui.text_edit_singleline(&mut state.user[idx].password);
                    });
                    ui.horizontal(|ui| {
                        ui.label("服务器选");
                        ui.radio_value(&mut state.user[idx].server, Server::Official, "官服");
                        ui.radio_value(&mut state.user[idx].server, Server::Bilibili, "B服");
                        if ui
                            .button(if state.user[idx].inherit {
                                "继承账号"
                            } else {
                                "独立设置"
                            })
                            .clicked()
                        {
                            state.user[idx].inherit = !state.user[idx].inherit;
                        }
                        // ui.label("账号");
                        if state.user[idx].inherit {
                            // ui.add(
                            //     DragValue::new(&mut state.user[idx].inherit_index).prefix("账号"),
                            // );
                            // ui.add(
                            //     DragValue::new(&mut state.user[idx].inherit_index)
                            //         .custom_formatter(|n, _| format!("账号{n}")),
                            // );
                            ui.add(DragValue::new(&mut state.user[idx].inherit_index));
                        }
                    });
                    ui.add_enabled_ui(!state.user[idx].inherit, |ui| {
                        Self::one_user(ui, state, idx);
                    });
                });
            })
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("明日方舟速通 611-12.01");
            });
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_sized(egui::vec2(WIDTH, 0.0), |ui: &mut egui::Ui| {
                    ui.vertical(|ui| {
                        egui::Grid::new("job").show(ui, |ui| {
                            ui.label("当前模式");

                            if ui
                                .button(if self.multi_account {
                                    "多号"
                                } else {
                                    "单号"
                                })
                                .clicked()
                            {
                                self.multi_account = !self.multi_account;
                            }

                            if ui
                                .button(if self.roguelike { "肉鸽" } else { "日常" })
                                .clicked()
                            {
                                self.roguelike = !self.roguelike;
                            }

                            ui.end_row();
                            if ui.button("高级设置").clicked() {}
                            if ui.button("帮助").clicked() {}
                            if ui.button("定时").clicked() {}
                            if ui.button("启动").clicked() {}
                            if ui.button("启动+定时").clicked() {}
                        });
                    })
                    .response
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_sized(
                    egui::vec2(WIDTH, ui.available_height()),
                    |ui: &mut egui::Ui| {
                        ui.vertical(|ui| {
                            if self.multi_account {
                                Self::multi_user(ui, self);
                            } else {
                                Self::single_user(ui, self);
                            }
                        })
                        .response
                    },
                )
            })
        });
    }
}
