use crate::api::{login_promise, LoginResult};
use crate::data::{Account, AccountBuilder, AccountMode, Server, Setting};
use chrono::{Local, NaiveDateTime, TimeZone, Timelike};
use egui::{Align, Area, DragValue, FontData, FontDefinitions, FontFamily, Key, Order, TextEdit};
use egui::{Button, Frame};
use egui_extras::{Column, TableBuilder};
use egui_notify::Toasts;
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 320.0;

#[derive(Deserialize, Serialize, PartialEq, Eq, Default, Clone)]
enum Layout {
    #[default]
    Account,
    Setting,
    Help,
}
impl Layout {
    fn toggle_default(&self, target: Layout) -> Self {
        if self != &target {
            target
        } else {
            Layout::default()
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct MyApp {
    account: Vec<Account>,
    setting: Setting,
    layout: Layout,
    scroll_to_account: usize,
    #[serde(skip)]
    logining: Option<Promise<LoginResult>>,
    #[serde(skip)]
    toast: Toasts,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut account = vec![AccountBuilder::default().inherit(false).build().unwrap()];
        account.extend(vec![AccountBuilder::default().build().unwrap(); 9999]);
        let total = account.len();
        let setting = Setting::default().multi_account_choice(format!("0-{total}"));
        Self {
            account,
            setting,
            layout: Layout::Account,
            scroll_to_account: 0,
            logining: None,
            toast: Default::default(),
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::set_style(&cc.egui_ctx);
        Default::default()
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
        fonts
            .families
            .entry(FontFamily::Monospace)
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

    fn one_account(ui: &mut egui::Ui, state: &mut Self, idx: usize) {
        if state.setting.multi_account {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(format!("#{}", idx)).clicked() {
                        let base = state
                            .setting
                            .multi_account_choice
                            .split("#")
                            .next()
                            .unwrap_or("")
                            .trim();
                        state.setting.multi_account_choice = format!("{base} #{idx}");
                    }
                })
            });
            ui.horizontal(|ui| {
                ui.label(format!("账号"));
                ui.text_edit_singleline(&mut state.account[idx].username);
            });
            ui.horizontal(|ui| {
                ui.label(format!("密码"));
                ui.text_edit_singleline(&mut state.account[idx].password);
            });
        }
        ui.horizontal(|ui| {
            ui.label("服务");
            ui.radio_value(&mut state.account[idx].server, Server::Official, "官服");
            ui.radio_value(&mut state.account[idx].server, Server::Bilibili, "B服");
            if !state.setting.multi_account {
                return;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(promise) = &state.logining {
                    if let Some(result) = promise.ready() {
                        let result = match result {
                            LoginResult::Fail => "无效",
                            LoginResult::Success => "有效",
                            LoginResult::Unknown => "未知",
                        };
                        state.toast.info(result);
                        // ui.label(result);
                        state.logining = None;
                    } else {
                        ui.spinner();
                    }
                    return;
                }

                let button = ui.button("测试");
                if button.clicked() {
                    // let promise = login_promise(
                    //     &state.account[idx].username,
                    //     &state.account[idx].password,
                    //     &state.account[idx].server,
                    // );
                    // state.logining = Some(promise);

                    state.toast.info("dddd");
                    state.toast.success("dddd");
                }
            });
        });
        ui.horizontal(|ui| {
            ui.label("模式");
            if ui.button(state.account[idx].mode.str()).clicked() {
                state.account[idx].mode = state.account[idx].mode.next();
            }

            if !state.setting.multi_account {
                return;
            }

            if state.account[idx].mode == AccountMode::Daily {
                if ui
                    .button(if state.account[idx].inherit {
                        "继承"
                    } else {
                        "独立"
                    })
                    .clicked()
                {
                    state.account[idx].inherit = !state.account[idx].inherit;
                }
                if state.account[idx].inherit {
                    let total = state.account.len();
                    ui.add(
                        DragValue::new(&mut state.account[idx].inherit_index)
                            .prefix("账号")
                            .clamp_range(0..=total - 1),
                    );
                }
            }
        });
        match state.account[idx].mode {
            AccountMode::Daily => {
                ui.add_enabled_ui(!state.account[idx].inherit, |ui| {
                    Self::one_account_daily(ui, state, idx);
                });
            }
            AccountMode::ZL => Self::one_account_zl(ui, state, idx),
            AccountMode::Recruit => Self::one_account_recruit(ui, state, idx),
        };
    }

    fn one_account_zl(ui: &mut egui::Ui, state: &mut Self, idx: usize) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut state.account[idx].zl_level, "等级(蜡烛)");
            ui.add(DragValue::new(&mut state.account[idx].zl_max_level).clamp_range(0..=9999));
            ui.checkbox(&mut state.account[idx].zl_coin, "源石锭");
            ui.add(DragValue::new(&mut state.account[idx].zl_max_coin).clamp_range(0..=9999));
        });
        ui.checkbox(&mut state.account[idx].zl_no_waste, "先做日常");
    }

    fn one_account_recruit(ui: &mut egui::Ui, state: &mut Self, idx: usize) {
        ui.horizontal(|ui| {
            ui.label("招募");
            let mut always_true = true;
            ui.add_enabled_ui(false, |ui| {
                ui.checkbox(&mut always_true, "其他");
            });
            ui.checkbox(&mut state.account[idx].recruit_recruit1, "小车");
            ui.checkbox(&mut state.account[idx].recruit_recruit4, "四星");
            ui.checkbox(&mut state.account[idx].recruit_recruit5, "五星");
            ui.checkbox(&mut state.account[idx].recruit_recruit6, "六星");
        });
    }

    fn one_account_daily(ui: &mut egui::Ui, state: &mut Self, idx: usize) {
        ui.add_enabled_ui(state.account[idx].job_fight, |ui| {
            ui.horizontal(|ui| {
                ui.label("关卡");
                ui.text_edit_singleline(&mut state.account[idx].fight);
            });
            ui.horizontal(|ui| {
                ui.label("吃药");
                ui.add(
                    DragValue::new(&mut state.account[idx].max_drug)
                        .clamp_range(0..=99)
                        .suffix("次"),
                );
                ui.label("石头");
                ui.add(
                    DragValue::new(&mut state.account[idx].max_stone)
                        .clamp_range(0..=99)
                        .suffix("次"),
                );
                ui.label("到期");

                let mut txt: String = state.account[idx]
                    .max_drug_day
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                let max_len = 18;
                if txt.len() > max_len {
                    txt = format!("{}...", txt.get(0..max_len - 3).unwrap())
                }

                let popup_id = ui.next_auto_id();
                let button_response = ui.add(Button::new(txt));
                // let button_response = ui.add(Button::new(egui::text::LayoutJob {
                // let mut job = LayoutJob::single_section(txt, TextFormat::default());
                // let mut job = LayoutJob::simple(txt, TextFormat::default());
                // job.wrap = TextWrapping {
                //     max_rows: 1,
                //     break_anywhere: true,
                //     max_width: 10.0,
                //     // overflow_character: "...".into(),
                //     ..Default::default()
                // };
                // let button_response = ui.add(Button::new(job));
                // egui::epaint::text::TextWrapping {
                //     // max_rows: 1,
                //     // break_anywhere: true,
                //     ..Default::default()
                // }
                // ));
                if button_response.clicked() {
                    ui.memory().toggle_popup(popup_id);
                }
                if ui.memory().is_popup_open(popup_id) {
                    let area_response = Area::new(popup_id)
                        .order(Order::Foreground)
                        // .fixed_pos(button_response.rect.max)
                        .fixed_pos(button_response.rect.min)
                        .constrain(true)
                        .show(ui.ctx(), |ui| {
                            Frame::popup(ui.style()).show(ui, |ui| {
                                // egui::Grid::new(ui.next_auto_id()).min_col_width(0.0).show(ui, |ui| {
                                egui::Grid::new("max_drug_day")
                                    .min_col_width(0.0)
                                    .show(ui, |ui| {
                                        let x = &mut state.account[idx].max_drug_day;
                                        let len = x.len();
                                        for i in 0..len {
                                            // ui.horizontal(|ui| {
                                            ui.label(format!("{}天", len - i - 1));
                                            ui.add(
                                                DragValue::new(
                                                    &mut state.account[idx].max_drug_day[i],
                                                )
                                                .suffix("个")
                                                .clamp_range(0..=99),
                                            );
                                            ui.end_row();
                                            // });
                                        }
                                    });
                            });
                        })
                        .response;

                    if !button_response.clicked()
                        && (ui.input().key_pressed(Key::Escape)
                            || area_response.clicked_elsewhere())
                    {
                        ui.memory().close_popup();
                    }
                }

                // egui::ComboBox::from_label("Select one!")
                //     .selected_text("ok")
                //     .show_ui(ui, |ui| {
                // ui.selectable_value(&mut selected, Enum::First, "First");
                // ui.selectable_value(&mut selected, Enum::Second, "Second");
                // ui.selectable_value(&mut selected, Enum::Third, "Third");
                // });
                // ui.text_edit_singleline(&mut state.account[idx].max_drug_day);
            });
        });
        ui.add_enabled_ui(state.account[idx].job_shop, |ui| {
            ui.horizontal(|ui| {
                ui.label("多买");
                let txt =
                    TextEdit::singleline(&mut state.account[idx].prefer_goods).desired_width(100.0);
                ui.add(txt);
                // ui.text_edit_singleline(&mut state.account[idx].prefer_goods);
                // });
                // ui.horizontal(|ui| {
                ui.label("少买");
                let txt = TextEdit::singleline(&mut state.account[idx].dislike_goods)
                    .desired_width(100.0);
                ui.add(txt);
            });
        });

        ui.add_enabled_ui(state.account[idx].job_recruit, |ui| {
            ui.horizontal(|ui| {
                ui.label("招募");
                ui.checkbox(&mut state.account[idx].recruit0, "其他");
                ui.checkbox(&mut state.account[idx].recruit1, "小车");
                ui.checkbox(&mut state.account[idx].recruit4, "四星");
                ui.checkbox(&mut state.account[idx].recruit5, "五星");
                ui.checkbox(&mut state.account[idx].recruit6, "六星");
            });
        });

        ui.horizontal(|ui| {
            ui.label("任务");

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.account[idx].job_mail, "邮件");
                    ui.checkbox(&mut state.account[idx].job_fight, "作战");
                    ui.checkbox(&mut state.account[idx].job_friend, "好友");
                    ui.checkbox(&mut state.account[idx].job_gain, "收菜");
                    ui.checkbox(&mut state.account[idx].job_shift, "换班");
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.account[idx].job_manu, "加速");
                    ui.checkbox(&mut state.account[idx].job_clue, "线索");
                    ui.checkbox(&mut state.account[idx].job_assist, "副手");
                    ui.checkbox(&mut state.account[idx].job_shop, "信交");
                    ui.checkbox(&mut state.account[idx].job_recruit, "公招");
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.account[idx].job_task, "任务");
                    ui.checkbox(&mut state.account[idx].job_activity, "活动");
                });
            });
        });

        ui.horizontal(|ui| {
            ui.label("时间");
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.account[idx].allow_monday, "周一");
                    ui.checkbox(&mut state.account[idx].allow_tuesday, "周二");
                    ui.checkbox(&mut state.account[idx].allow_wednesday, "周三");
                    ui.checkbox(&mut state.account[idx].allow_thursday, "周四");
                    ui.checkbox(&mut state.account[idx].allow_friday, "周五");
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.account[idx].allow_saturday, "周六");
                    ui.checkbox(&mut state.account[idx].allow_sunday, "周日");
                    let txt = TextEdit::singleline(&mut state.account[idx].allow_after)
                        .desired_width(120.0);
                    let response = ui.add(txt);
                    if response.lost_focus() {
                        let dt = &state.account[idx].allow_after;
                        let dt = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M")
                            .map(|dt| Local.from_local_datetime(&dt).unwrap())
                            .unwrap_or(
                                Local::now()
                                    .with_hour(0)
                                    .and_then(|x| x.with_minute(0))
                                    .unwrap(),
                            );
                        state.account[idx].allow_after = dt.format("%Y-%m-%d %H:%M").to_string()
                    }
                    ui.label("起");

                    // use chrono::{offset::Utc, DateTime, NaiveDate, NaiveDateTime, NaiveTime};
                    // use egui_datepicker::DatePicker;

                    // let mut date = Utc::now();
                    // let date: DateTime<Utc> = Utc::now();
                    // let mut date: chrono::naive::NaiveDateTime = Utc::now().naive_utc();

                    // ui.add(DatePicker::<RangeInclusive<NaiveDateTime>>::new("super_unique_id", &mut date));
                    // ui.add(DatePicker::new("datepicker-unique-id", &mut date));
                });
            });
        });
    }

    fn single_account(ui: &mut egui::Ui, state: &mut Self) {
        let idx = 0;

        Self::one_account(ui, state, idx);
    }

    fn multi_account(ui: &mut egui::Ui, state: &mut Self, scroll_to_account_changed: bool) {
        let row_height = HEIGHT;
        let mut table = TableBuilder::new(ui).column(Column::initial(WIDTH));
        if scroll_to_account_changed {
            table = table.scroll_to_row(state.scroll_to_account, None);
        }

        table.body(|body| {
            body.rows(row_height, state.account.len(), |row_index, mut row| {
                row.col(|ui| {
                    let idx = row_index;
                    Self::one_account(ui, state, idx);
                });
            })
        });
    }

    fn setting(ui: &mut egui::Ui, state: &mut Self) {
        ui.horizontal(|ui| {
            ui.label("图鉴账号");
            ui.text_edit_singleline(&mut state.setting.captcha_username);
        });
        ui.horizontal(|ui| {
            ui.label("图鉴密码");
            ui.text_edit_singleline(&mut state.setting.captcha_password);
        });
        ui.horizontal(|ui| {
            ui.label("同一关卡连续导航或代理失败出现");
            ui.add(
                DragValue::new(&mut state.setting.max_fight_failed_times)
                    .suffix("次")
                    .clamp_range(0..=99),
            );
            ui.label("后跳过");
        });
        ui.horizontal(|ui| {
            ui.label("同一账号登录界面15分钟内出现");
            ui.add(
                DragValue::new(&mut state.setting.max_login_times_15min)
                    .suffix("次")
                    .clamp_range(0..=99),
            );
            ui.label("后跳过");
        });
        // ui.horizontal(|ui| {
        //     ui.label("同一账号6至0天理智药分别吃");
        //     let txt =
        //         TextEdit::singleline(&mut state.setting.max_drug_times_day).desired_width(100.0);
        //     ui.add(txt);
        //     ui.label("个")
        // });
        ui.horizontal(|ui| {
            ui.label("通知账号");
            ui.text_edit_singleline(&mut state.setting.qq_notify);
        });
        ui.horizontal(|ui| {
            ui.label("通知服务");
            ui.text_edit_singleline(&mut state.setting.qq_notify_server);
        });
        ui.horizontal(|ui| {
            ui.label("通知场景");
            // egui::Grid::new("qq_notify_scene").show(ui, |ui| {
            ui.checkbox(&mut state.setting.qq_notify_mail, "邮件前");
            ui.checkbox(&mut state.setting.qq_notify_dorm_enter, "进基建");
            ui.checkbox(&mut state.setting.qq_notify_dorm_leave, "出基建");
            ui.checkbox(&mut state.setting.qq_notify_task, "任务前");
            // });
            // ui.checkbox(checked, text)
            // ui.text_edit_singleline(&mut state.setting.captcha_password);
        });
        ui.horizontal(|ui| {
            ui.label("多号模式");
            ui.checkbox(&mut state.setting.multi_account, "");
        });

        // let txt = if self.multi_account {
        //     "多号"
        // } else {
        //     "单号"
        // };
        // if ui.button(txt).clicked() {
        //     self.multi_account = !self.multi_account;
        // }
        ui.horizontal(|ui| {
            ui.label("多号线索账号");
            ui.text_edit_singleline(&mut state.setting.multi_account_clue);
        });
        ui.horizontal(|ui| {
            ui.label("多号不跳过空白账号");
            ui.checkbox(&mut state.setting.multi_account_allow_empty, "");
        });

        ui.horizontal(|ui| {
            ui.label("定时重启");
            ui.text_edit_singleline(&mut state.setting.crontab)
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.toast.show(ctx);

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Mizuki 611-12.01");
            });
        });

        let mut scroll_to_account_changed = false;
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_sized(egui::vec2(WIDTH, 0.0), |ui: &mut egui::Ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(self.layout == Layout::default(), |ui| {
                                if ui.button("启动").clicked() {}
                            });
                            if ui.button("设置").clicked() {
                                self.layout = self.layout.toggle_default(Layout::Setting);
                            }
                            ui.add_enabled_ui(self.layout == Layout::default(), |ui| {
                                ui.add_visible_ui(self.setting.multi_account, |ui| {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| {
                                            scroll_to_account_changed = ui
                                                .add(
                                                    DragValue::new(&mut self.scroll_to_account)
                                                        .clamp_range(0..=self.account.len() - 1)
                                                        .prefix("跳转至"),
                                                )
                                                .changed();
                                            ui.label("启用");
                                            ui.add(TextEdit::singleline(
                                                &mut self.setting.multi_account_choice,
                                            ));
                                        },
                                    )
                                });
                            });
                        });
                    })
                    .response
                });
            });
        });
        if self.account.len() < 1 {
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_sized(
                    egui::vec2(WIDTH, ui.available_height()),
                    |ui: &mut egui::Ui| {
                        ui.vertical(|ui| match self.layout {
                            Layout::Setting => Self::setting(ui, self),
                            Layout::Help => Self::setting(ui, self),
                            Layout::Account => {
                                if self.setting.multi_account {
                                    Self::multi_account(ui, self, scroll_to_account_changed)
                                } else {
                                    Self::single_account(ui, self)
                                }
                            }
                        })
                        .response
                    },
                )
            })
        });
    }
}
