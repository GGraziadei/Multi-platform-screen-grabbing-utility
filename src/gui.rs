use std::sync::{Arc, Mutex, RwLock};
use std::thread::{JoinHandle, spawn};
use egui::{Key, ScrollArea};
use eframe::Result;
use egui::accesskit::Role::Image;
use screenshots::DisplayInfo;
use crate::configuration::{Configuration, ImageFmt};
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshots::ScreenshotExecutor;

struct Content {
    configuration : Arc<RwLock<Configuration>>,
    screenshot_executor : ScreenshotExecutor,
    encoders : Arc<Mutex<Vec<EncoderThread>>>,
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Press/Hold/Release example. Press A to test.");
            if ui.button("Clear").clicked() {
                self.text.clear();
            }

            if ui.button("Screenshot").clicked() {
                let di = DisplayInfo::from_point(0,0);
                let image = self.screenshot_executor.screenshot(di.unwrap(), None, None);
                ImageFormatter::from(image.unwrap()).to_clipboard().unwrap();
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
                self.text.push_str("\nTake screenshot");
                ui.ctx().request_repaint(); // make sure we note the holding.
                let mut encoders = self.encoders.lock()
                    .expect("Error in encoders access");
                let di = DisplayInfo::from_point(0,0);
                let image = self.screenshot_executor.screenshot(di.unwrap(), None, None);

                encoders.push(ImageFormatter::from(image.unwrap()).save_fmt("target/ui_test".to_string(), ImageFmt::PNG));
                drop(encoders);

            }
            if ctx.input(|i| i.key_released(Key::A)) {
                self.text.push_str("\nReleased");
            }
        });
    }
}

/*Marker struct. The event loop is executed over the main thread.*/
pub struct GuiThread;

impl GuiThread {

    pub fn new(configuration: Arc<RwLock<Configuration>>, encoders: Arc<Mutex<Vec<EncoderThread>>>, s : ScreenshotExecutor) -> Self
    {
        let configuration_read = configuration.read()
            .expect("Error. Cannot run gui thread without configuration file.");

        let app_name_tmp = configuration_read.get_app_name().unwrap().clone();

        drop(configuration_read);

        let options = eframe::NativeOptions::default();
        let content = Content{
            configuration,
            screenshot_executor: s,
            encoders,
            text: "".to_string(),
        };

        eframe::run_native(
            &*app_name_tmp,
            options,
            Box::new(move |_cc| Box::<Content>::new(content)),
        ).expect("Error during gui thread init.");

        Self
    }

}

