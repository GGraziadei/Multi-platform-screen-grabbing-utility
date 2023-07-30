use std::default::Default;
use std::sync::{Arc, Mutex, RwLock};
use eframe::run_native;
use egui::*;
use egui::{RichText};
use egui_extras::RetainedImage;
use screenshots::{Compression, DisplayInfo};
use crate::configuration::Configuration;
use crate::configuration::ImageFmt::PNG;
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshot_window::screenshot_window;
use crate::screenshots::{CaptureArea, ScreenshotExecutor};

struct Content {
  configuration : Arc<RwLock<Configuration>>,
  screenshot_executor : ScreenshotExecutor,
  encoders : Arc<Mutex<Vec<EncoderThread>>>,
  text: String,
  close: bool
}

impl eframe::App for Content {
  fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    let window_size = _frame.info().window_info.size;
    
    if self.close {
      _frame.set_visible(false);
    }

    let bg_color = ctx.style().visuals.panel_fill;

    TopBottomPanel::top("top")
      .frame(Frame{fill: bg_color, ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .min_height(window_size.y*0.1)
      .show(ctx, |ui| {
        ui.allocate_ui_with_layout(
          Vec2::new(window_size.x, window_size.y*0.1),
          Layout::centered_and_justified(Direction::TopDown),
          |ui| {
            ui.heading("Acquisisci una nuova schermata")
          }
        );
      });
    TopBottomPanel::top("bottom")
      .frame(Frame{fill: bg_color, ..Default::default()})
      .show_separator_line(false)
      .resizable(false)
      .min_height(window_size.y*0.9)
      .show(ctx, |ui| {
        let w = 0.3;
        SidePanel::left("left")
          .frame(Frame{inner_margin: Margin::symmetric(20.0, 0.0), fill: bg_color, ..Default::default()})
          .show_separator_line(false)
          .resizable(false)
          .exact_width(window_size.x*w)
          .show(ctx, |ui| {
            ui.allocate_ui_with_layout(
              Vec2::new(window_size.x*w, window_size.y*0.9),
              Layout::top_down_justified( Align::Center),
              |ui| {
                ui.label(RichText::new("Modalità di acquisizione").size(16.0));
                ui.add_space(10.0);
                ui.spacing_mut().button_padding = Vec2::new(10.0, 10.0);
                ui.spacing_mut().item_spacing.y = 10.0;
                if ui.button("Regione rettangolare").clicked(){};
                if ui.button("Tutti gli schermi").clicked(){};
                if ui.button("Schermo attuale").clicked(){
                  let di = DisplayInfo::from_point(0,0).unwrap();
                  let screenshot = self.screenshot_executor.screenshot(di, None, CaptureArea::new(0,0, di.width, di.height)).unwrap();
                  let imgf = screenshot.to_png(Some(Compression::Best)).unwrap();
                  let mut encoders = self.encoders.lock().unwrap();
                  encoders.push(ImageFormatter::from(screenshot).save_fmt("target/ui_test".to_string(), PNG));
                  ctx.memory_mut(|mem|{
                    mem.data.insert_temp(Id::from("screenshot"), imgf);
                    // screenshot_window(self.configuration.clone(), self.encoders.clone(), ScreenshotExecutor::new().0);
                  });
                };
                if ui.button("Finestra attiva").clicked(){};
                if ui.button("Finestra sotto al cursore").clicked(){};
              }
            );
          });
        SidePanel::right("right")
          .frame(Frame{inner_margin: Margin::symmetric(20.0, 0.0), fill: bg_color, ..Default::default()})
          .show_separator_line(false)
          .resizable(false)
          .exact_width(window_size.x*(1.0-w))
          .show(ctx, |ui| {
            ui.allocate_ui_with_layout(
              Vec2::new(window_size.x*(1.0-w), window_size.y*0.8),
              Layout::top_down( Align::LEFT),
              |ui| {
                ui.spacing_mut().item_spacing.y = 10.0;
                ui.label(RichText::new("Opzioni di acquisizione").size(16.0));
                ui.checkbox(&mut true, "Includi il puntatore del mouse");
                ui.checkbox(&mut true, "Includi la barra del titolo e i bordi della finestra");
                ui.checkbox(&mut true, "Cattura solo la finestra attuale");
                ui.checkbox(&mut true, "Esci dopo il salvataggio o la copia manuali");
                ui.checkbox(&mut true, "Cattura al click");
                let mut painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("screenshot")));
                let size_x = (window_size.x*(1.0-w))-80.0;
                let size_y = size_x*9.0/16.0;
                let mut r_image = RetainedImage::from_color_image("placeholder", ColorImage::example());
                let mut screenshot_done = false;
                ctx.memory(|mem|{
                  let image = mem.data.get_temp::<Vec<u8>>(Id::from("screenshot"));
                  if image.is_some(){
                    r_image = RetainedImage::from_image_bytes("screenshot", image.unwrap().as_slice()).unwrap();
                    screenshot_done = true;
                  }
                });
                if screenshot_done {
                  _frame.close();
                  screenshot_window(self.configuration.clone(), self.encoders.clone(), ScreenshotExecutor::new().0);
                  println!("CHIUSA");
                }
              });
          });
      });
  }
  
  fn on_close_event(&mut self) -> bool {
    self.close = true;
    false
  }
}

pub fn main_window(configuration: Arc<RwLock<Configuration>>, encoders: Arc<Mutex<Vec<EncoderThread>>>, s : ScreenshotExecutor){
  let configuration_read = configuration.read()
    .expect("Error. Cannot run gui thread without configuration file.");
  
  let app_name_tmp = configuration_read.get_app_name().unwrap().clone();
  
  drop(configuration_read);
  
  let options = eframe::NativeOptions{
    resizable: false,
    follow_system_theme: true,
    initial_window_size: Some(egui::Vec2::new(640.0, 360.0)),
    ..Default::default()
  };
  
  let content = Content{
    configuration,
    screenshot_executor: s,
    encoders,
    text: "".to_string(),
    close: false,
  };
  
  run_native(
    &*app_name_tmp,
    options,
    Box::new(move |_cc| Box::<Content>::new(content)),
  ).expect("Error during gui thread init.");
}
