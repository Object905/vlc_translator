use std::process::{Command, Stdio};

use rustc_serialize::json::Json;
use regex::Regex;

use notify_rust::Notification;
use notify_rust::NotificationUrgency::Critical;

use settings;

lazy_static! {
        static ref API_KEY: String =
            "trnsl.1.1.20160929T112555Z.acb10979dce357b0.50b33e6728c28c8cf3ce73780a18a82f9d22fba0"
            .to_owned();

        static ref DIRECTION_LANG: String = {
            if *&settings::SOURCE_LANG == "auto" {
                
                settings::TARGET_LANG.to_owned()
            } else {
                format!("{src}-{target}",
                    src = *&settings::SOURCE_LANG,
                    target = *&settings::TARGET_LANG)
            }
        };

        static ref GOOGLE_TRANSLATOR_URL: String = {
            format!("https://translate.google.ru/?hl=en#{src}/{target}/",
            src = *&settings::SOURCE_LANG,
            target = *&settings::TARGET_LANG)
        };

        static ref TAGS: Regex = Regex::new(r"<[^>]*?>").unwrap();
        static ref NEWLINES: Regex = Regex::new(r"\r\n|\n").unwrap();
}

pub fn translate(text: &str) -> String {
    let url = format!("https://translate.yandex.net/api/v1.5/tr.json/translate?\
        key={api_key}&lang={lang}&text={text}",
                      api_key = *API_KEY,
                      lang = *DIRECTION_LANG,
                      text = text);

    let output = Command::new("curl").arg(&url).output().unwrap();
    let response = String::from_utf8_lossy(&output.stdout).to_lowercase();

    let json = Json::from_str(&response).unwrap();

    let text = json.find("text").unwrap()[0]
    .as_string().unwrap() // first from array as string
    .trim_left_matches('"')
    .trim_right_matches('"'); // remove quotes

    text.to_owned()
}

pub fn show_desktop_notification(summary: &str, body: &str) {
    let time: i32;
    if body.len() <= 15 {
        time = 2500;
    } else {
        time = (2500 + (body.len() - 15) * 100) as i32;
    }

    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(time)
        .urgency(Critical)
        .show()
        .unwrap();
}

pub fn open_google_translator(text: &str) {
    Command::new(settings::BROWSER_COMMAND)
        .stdout(Stdio::null())
        .arg(GOOGLE_TRANSLATOR_URL.to_owned() + text)
        .status()
        .unwrap();
}

pub fn prepare_text(text: &str) -> String {
    let removed_tags = TAGS.replace_all(&text, "");
    NEWLINES.replace_all(&removed_tags, " ")
}
