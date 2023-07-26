use std::fs;
use std::time::Duration;
use ::screenshots::{Compression, DisplayInfo, Screen};
use serde::de::Unexpected::Option;

use crate::configuration::Configuration;
use crate::screenshots::ScreenshotExecutor;

mod configuration;
mod screenshots;

fn main() {
    let c = Configuration::new();
    c.set_delay(Some(Duration::from_secs(10)));

    let s = ScreenshotExecutor::new();
    let di = Screen::all().unwrap()[0].display_info;
    let delay = c.get_delay();

    let image = s.screenshot(di,delay).unwrap();
    let mut buffer = image.to_png(None).unwrap();
    fs::write(format!("target/screen_test.png"), buffer).unwrap();
}