use std::default::Default;
use std::sync::{Arc, Mutex, RwLock};
use eframe::{egui, run_native, Theme};
use egui::*;
use egui_modal::Modal;
use crate::configuration::Configuration;
use crate::image_formatter::{EncoderThread};
use crate::screenshots::{ScreenshotExecutor};

pub enum WindowType {
  Main,
  Settings,
  Screenshot,
  Select
}

pub struct Content {
  configuration : Arc<RwLock<Configuration>>,
  screenshot_executor : ScreenshotExecutor,
  encoders : Arc<Mutex<Vec<EncoderThread>>>,
  text: String,
  close: bool,
  window_type: WindowType
}

impl Content {
  pub fn get_se(&self) -> &ScreenshotExecutor {
    &self.screenshot_executor
  }
  pub fn get_enc(&self) -> &Arc<Mutex<Vec<EncoderThread>>> {
    &self.encoders
  }
	pub fn set_win_type(&mut self, t: WindowType) {
		self.window_type = t;
	}
}

impl eframe::App for Content {
  fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    match self.window_type{
      WindowType::Main => self.main_window(ctx, _frame),
      // WindowType::Settings => self.settings_window(ctx, _frame),
      WindowType::Screenshot => self.screenshot_window(ctx, _frame),
      WindowType::Select => self.select_window(ctx, _frame),
      _ => {}
    }
  }
}

pub fn draw_window(configuration: Arc<RwLock<Configuration>>, encoders: Arc<Mutex<Vec<EncoderThread>>>, s : ScreenshotExecutor){
  let configuration_read = configuration.read()
    .expect("Error. Cannot run gui thread without configuration file.");

  let app_name_tmp = configuration_read.get_app_name().unwrap().clone();

  drop(configuration_read);

  let options = eframe::NativeOptions{
    resizable: false,
    follow_system_theme: true,
    initial_window_size: Some(Vec2::new(640.0, 360.0)),
    default_theme: Theme::Dark,
    // centered: true,
    ..Default::default()
  };

  let content = Content{
    configuration,
    screenshot_executor: s,
    encoders,
    text: "".to_string(),
    close: false,
    window_type: WindowType::Main,
  };

  run_native(
    &*app_name_tmp,
    options,
    Box::new(move |_cc| Box::<Content>::new(content)),
  ).expect("Error during gui thread init.");
}
