use ::screenshots::{DisplayInfo};
use image::ImageResult;
use log::warn;

use crate::configuration::{Configuration, ImageFmt};
use crate::image_formatter::ImageFormatter;
use crate::screenshots::{CaptureArea, ScreenshotExecutor};

mod configuration;
mod screenshots;
mod image_formatter;

fn main() {

    env_logger::init();

    let c = Configuration::new();
    c.set_delay(None);
    c.set_image_fmt(ImageFmt::GIF);

    let s = ScreenshotExecutor::new();
    let di = DisplayInfo::from_point(0,0).unwrap();
    let delay = c.get_delay();

    let image = s.screenshot(di,delay, CaptureArea::new(0,0,720,720)).unwrap();
    drop(s);

    //fs::write(format!("target/screen_test.png"), buffer).unwrap();
    //image::save_buffer_with_format("target/test.png", image.rgba(), image.width(), image.height(), ColorType::Rgba8, c.get_image_fmt().unwrap().get_image_format().unwrap()).expect("TODO: panic message");
    let img_fmt = ImageFormatter::from(image);
    let t =  img_fmt.save_fmt("target/test".to_string(), c.get_image_fmt().unwrap());

    warn!("New screen. non attendi la fine del precednete encoding");


    img_fmt.to_clipboard().unwrap();

    t.join().unwrap();
}