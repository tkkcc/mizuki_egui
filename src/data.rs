use chrono::{Local, Timelike};
use derivative::Derivative;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Default, Clone)]
pub enum Server {
    #[default]
    Official,
    Bilibili,
}
impl Server {
    #[allow(dead_code)]
    pub fn next(self) -> Self {
        match self {
            Self::Official => Self::Bilibili,
            Self::Bilibili => Self::Official,
        }
    }
    #[allow(dead_code)]
    pub fn str(self) -> String {
        match self {
            Self::Official => "官服",
            Self::Bilibili => "B服",
        }
        .into()
    }
}

#[derive(Deserialize, Serialize, PartialEq, Default, Clone)]
pub enum AccountMode {
    #[default]
    Daily,
    ZL,
    Recruit,
}

impl AccountMode {
    pub fn next(&self) -> Self {
        match self {
            Self::Daily => Self::ZL,
            Self::ZL => Self::Recruit,
            Self::Recruit => Self::Daily,
        }
    }
    pub fn str(&self) -> String {
        match self {
            Self::Daily => "日常",
            Self::ZL => "肉鸽",
            Self::Recruit => "公招",
        }
        .into()
    }
}

#[derive(Builder, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
#[builder(default)]
pub struct Account {
    #[builder(default = "9999")]
    pub zl_max_coin: usize,
    #[builder(default = "9999")]
    pub zl_max_level: usize,
    #[builder(default = "true")]
    pub zl_coin: bool,
    #[builder(default = "true")]
    pub zl_level: bool,
    #[builder(default = "true")]
    pub zl_no_waste: bool,
    pub mode: AccountMode,
    #[builder(default = "true")]
    pub inherit: bool,
    pub inherit_index: usize,
    pub username: String,
    pub password: String,
    pub server: Server,
    #[builder(default = "\"jm hd ce ls ap pr\".to_string()")]
    pub fight: String,
    pub max_drug: usize,
    #[builder(default = "vec![0,1,1,1,9,9,99]")]
    pub max_drug_day: Vec<usize>,
    pub max_stone: usize,
    pub prefer_goods: String,
    pub dislike_goods: String,

    #[builder(default = "true")]
    pub recruit0: bool,
    #[builder(default = "true")]
    pub recruit1: bool,
    #[builder(default = "true")]
    pub recruit4: bool,
    #[builder(default = "true")]
    pub recruit5: bool,
    #[builder(default = "true")]
    pub recruit6: bool,

    #[builder(default = "true")]
    pub recruit_recruit1: bool,
    #[builder(default = "true")]
    pub recruit_recruit4: bool,
    #[builder(default = "true")]
    pub recruit_recruit5: bool,
    #[builder(default = "true")]
    pub recruit_recruit6: bool,
    #[builder(default = "true")]
    pub job_mail: bool,
    #[builder(default = "true")]
    pub job_fight: bool,
    #[builder(default = "true")]
    pub job_friend: bool,
    #[builder(default = "true")]
    pub job_gain: bool,
    #[builder(default = "true")]
    pub job_shift: bool,
    #[builder(default = "true")]
    pub job_manu: bool,
    #[builder(default = "true")]
    pub job_clue: bool,
    #[builder(default = "true")]
    pub job_assist: bool,
    #[builder(default = "true")]
    pub job_shop: bool,
    #[builder(default = "true")]
    pub job_recruit: bool,
    #[builder(default = "true")]
    pub job_task: bool,
    #[builder(default = "true")]
    pub job_activity: bool,
    #[builder(default = "true")]
    pub allow_monday: bool,
    #[builder(default = "true")]
    pub allow_tuesday: bool,
    #[builder(default = "true")]
    pub allow_wednesday: bool,
    #[builder(default = "true")]
    pub allow_thursday: bool,
    #[builder(default = "true")]
    pub allow_friday: bool,
    #[builder(default = "true")]
    pub allow_saturday: bool,
    #[builder(default = "true")]
    pub allow_sunday: bool,

    // #[builder(default = "\"2022-01-01 00:00\".to_string()")]
    #[builder(
        default = "Local::now().with_minute(0).and_then(|x| x.with_hour(0)).unwrap().format(\"%Y-%m-%d %H:%M\").to_string()"
    )]
    pub allow_after: String,
}

#[derive(Deserialize, Serialize, Clone, Derivative)]
#[derivative(Default)]
#[serde(default)]
pub struct Setting {
    pub multi_account: bool,
    #[derivative(Default(value = "\"0-9999\".to_string()"))]
    pub multi_account_choice: String,
    pub captcha_username: String,
    pub captcha_password: String,
    #[derivative(Default(value = "3"))]
    pub max_login_times_15min: usize,
    #[derivative(Default(value = "2"))]
    pub max_fight_failed_times: usize,
    pub qq_notify: String,
    pub qq_notify_server: String,
    #[derivative(Default(value = "true"))]
    pub qq_notify_mail: bool,
    #[derivative(Default(value = "true"))]
    pub qq_notify_dorm_enter: bool,
    #[derivative(Default(value = "true"))]
    pub qq_notify_dorm_leave: bool,
    #[derivative(Default(value = "true"))]
    pub multi_account_allow_empty: bool,
    pub qq_notify_task: bool,
    pub multi_account_clue: String,

    #[derivative(Default(value = "\"4:00 12:00 20:00\".to_string()"))]
    pub crontab: String,
}

impl Setting {
    pub fn multi_account_choice(mut self, x: String) -> Self {
        self.multi_account_choice = x.into();
        self
    }
}
