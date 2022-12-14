use chrono::Utc;
use ehttp::{self, Request};
use poll_promise::Promise;
use poll_promise::Sender;
use rsa::pkcs8::DecodePublicKey;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

use crate::data::Server;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum LoginResult {
    Success,
    Fail,
    Unknown,
}
use self::LoginResult::*;
pub fn official_login_request(username: &str, password: &str) -> Request {
    #[derive(Serialize)]
    struct Body {
        phone: String,
        password: String,
    }
    let body = Body {
        phone: username.to_string(),
        password: password.to_string(),
    };
    let body = serde_json::to_string(&body).unwrap_or("".into());

    let request = ehttp::Request::post(
        "https://as.hypergryph.com/user/auth/v1/token_by_phone_password",
        body.as_bytes().to_vec(),
    );
    request
}

pub fn official_login_promise(username: &str, password: &str) -> Promise<LoginResult> {
    let (sender, promise) = Promise::new();
    let request = official_login_request(username, password);
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        fn f(result: ehttp::Result<ehttp::Response>) -> Result<LoginResult, Box<dyn Error>> {
            #[cfg(test)]
            println!("{}", result.as_ref().unwrap().text().unwrap());

            let response = result?;
            let r = response.text().ok_or("")?;
            let r: Value = serde_json::from_str(r)?;
            let code = r["status"].as_i64().unwrap_or(-1);
            Ok(match code {
                0 => Success,
                100 => Fail,
                _ => Unknown,
            })
        }
        let result = f(result).unwrap_or(Unknown);
        sender.send(result);
    });
    promise
}

pub fn bilibili_login_second(
    result: ehttp::Result<ehttp::Response>,
    sender: Sender<LoginResult>,
    username: &str,
    password: &str,
) -> Result<LoginResult, Box<dyn Error>> {
    let response = result?;
    let r = response.text().ok_or("")?;
    let r: Value = serde_json::from_str(r)?;
    let hash = r["data"]["hash"].as_str().ok_or("")?;
    let key = r["data"]["key"].as_str().ok_or("")?;

    use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
    let key = RsaPublicKey::from_public_key_pem(key)?;

    let password = format!("{}{}", hash, password);

    let mut rng = rand::thread_rng();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let password = key.encrypt(&mut rng, padding, password.as_bytes())?;
    let password = base64::encode(password);

    use chrono::DateTime;
    let time = Utc::now().timestamp();
    let appkey = "bca7e84c2d947ac6";
    let appsec = "60698ba2f68e01ce44738920a0ffe768";
    let url = "https://passport.bilibili.com/x/passport-login/oauth2/login";
    let body = [
        ("actionKey", "appkey"),
        ("appkey", appkey),
        ("build", &6270200.to_string()),
        ("captcha", ""),
        ("challenge", ""),
        ("channel", "bili"),
        ("device", "phone"),
        ("mobi_app", "android"),
        ("password", &password),
        ("permission", "ALL"),
        ("platform", "android"),
        ("seccode", ""),
        ("subid", &1.to_string()),
        ("ts", &time.to_string()),
        ("username", username),
        ("validate", ""),
    ];
    use url::form_urlencoded;
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(body)
        .finish();
    let md5_string = format!("{}{}", encoded, appsec);
    let digest = md5::compute(md5_string);
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(body)
        .append_pair("sign", &format!("{:x}", digest))
        .finish();

    let request = Request {
        method: "POST".into(),
        url: url.into(),
        body: encoded.as_bytes().to_vec(),
        headers: [(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        )]
        .into(),
    };

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        fn f(result: ehttp::Result<ehttp::Response>) -> Result<LoginResult, Box<dyn Error>> {
            #[cfg(test)]
            println!("{}", result.as_ref().unwrap().text().unwrap());

            let response = result?;
            let r = response.text().ok_or("")?;
            let r: Value = serde_json::from_str(r)?;
            let code = r["code"].as_i64().unwrap_or(-1);
            Ok(match code {
                -629 => Fail,
                0 => Success,
                _ => Unknown,
            })
        }
        let result = f(result).unwrap_or(Unknown);
        sender.send(result);
    });

    Ok(Success)
}

pub fn bilibili_login_promise(username: &str, password: &str) -> Promise<LoginResult> {
    let (sender, promise) = Promise::new();
    let request = Request::get("https://passport.bilibili.com/x/passport-login/web/key");
    let username = username.to_owned();
    let password = password.to_owned();
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        bilibili_login_second(result, sender, &username, &password).ok();
    });
    promise
}

pub fn login_promise(username: &str, password: &str, server: &Server) -> Promise<LoginResult> {
    match server {
        Server::Official => official_login_promise(username, password),
        Server::Bilibili => bilibili_login_promise(username, password),
    }
}

pub fn login(username: &str, password: &str, server: &Server) -> LoginResult {
    let promise = login_promise(username, password, server);
    let result = promise.block_until_ready();
    *result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn normal() {
        assert_eq!(
            login("16517816184", "13724362620ABC", &Server::Official),
            Success
        );
        assert_eq!(login("16517", "13724362620ABC", &Server::Official), Fail);

        assert_eq!(
            login("17803229160", "beiqi780416", &Server::Bilibili),
            Success
        );
        assert_eq!(login("abc", "abc", &Server::Bilibili), Fail);
    }
}
