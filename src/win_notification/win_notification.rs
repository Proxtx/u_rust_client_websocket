use serde::Deserialize;
use winrt_notification::{Duration, Sound, Toast};

#[derive(Deserialize)]
pub struct WinNotificationConfiguration {
    title: Option<String>,
    text1: Option<String>,
    text2: Option<String>,
    sound: Option<String>,
    duration: Option<bool>,
}

pub struct WinNotification {}

impl WinNotification {
    pub fn new() -> Self {
        WinNotification {}
    }

    pub fn notification(&self, config: WinNotificationConfiguration) -> bool {
        let mut t = Toast::new(Toast::POWERSHELL_APP_ID);
        if let Some(_) = config.title {
            t = t.title(&config.title.unwrap());
        }
        if let Some(_) = config.text1 {
            t = t.text1(&config.text1.unwrap());
        }
        if let Some(_) = config.text2 {
            t = t.text1(&config.text2.unwrap());
        }
        if let Some(_) = config.sound {
            match config.sound.unwrap().as_str() {
                "SMS" => t = t.sound(Some(Sound::SMS)),
                "Mail" => t = t.sound(Some(Sound::Mail)),
                "Reminder" => t = t.sound(Some(Sound::Reminder)),
                "Default" => t = t.sound(Some(Sound::Default)),
                _ => t = t.sound(Some(Sound::Default)),
            };
        }

        if let Some(_) = config.duration {
            if config.duration.unwrap() {
                t = t.duration(Duration::Long);
            }
        } else {
            t = t.duration(Duration::Short);
        }

        match t.show() {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
