use ::screenshots::{DisplayInfo};
use egui::{Key, ScrollArea};
use image::ImageResult;
use log::{error, warn};

use crate::configuration::{Configuration, ImageFmt};
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshots::{CaptureArea, ScreenshotExecutor, ScreenshotExecutorThread};
use crate::thread_manager::ThreadManager;

mod configuration;
mod screenshots;
mod image_formatter;
mod thread_manager;


#[derive(Default)]
struct Content {
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Press/Hold/Release example. Press A to test.");
            if ui.button("Clear").clicked() {
                self.text.clear();
            }
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.label(&self.text);
                });

            if ctx.input(|i| i.key_pressed(Key::A)) {
                self.text.push_str("\nPressed");
            }
            if ctx.input(|i| i.key_down(Key::A)) {
                self.text.push_str("\nHeld");
                ui.ctx().request_repaint(); // make sure we note the holding.
            }
            if ctx.input(|i| i.key_released(Key::A)) {
                self.text.push_str("\nReleased");
            }
        });
    }
}

fn main() {

    env_logger::init();
    let c = Configuration::new();
    c.set_delay(None);
    c.set_image_fmt(ImageFmt::PNG);

    let (s, mut tm) = ThreadManager::new();
    let di = DisplayInfo::from_point(0,0).unwrap();
    let delay = c.get_delay();

    let image = s.screenshot(di,delay, CaptureArea::new(0,0,720,720)).unwrap();
    drop(s);

    let img_fmt = ImageFormatter::from(image);
    tm.add_encoder(img_fmt.save_fmt("target/test".to_string(), c.get_image_fmt().unwrap()));

    warn!("New screen. non attendi la fine del precednete encoding");


    img_fmt.to_clipboard().unwrap();

    let options = eframe::NativeOptions::default();
    let res = eframe::run_native(
        &*c.get_app_name().unwrap(),
        options,
        Box::new(|_cc| Box::<Content>::default()),
    );

    tm.join();



}