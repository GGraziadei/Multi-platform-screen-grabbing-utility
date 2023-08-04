use std::default::Default;
use std::sync::{Arc, Mutex, RwLock};
use eframe::{egui, run_native, Theme};
use egui::*;
use egui_modal::Modal;
use log::error;
use mouse_position::mouse_position::Mouse;
use screenshots::DisplayInfo;
use crate::configuration::{Configuration, ImageFmt};
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshots::{ScreenshotExecutor};
use WindowType::*;
use crate::image_combiner::ImageCombiner;

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
  window_type: WindowType,
  region: Option<Rect>,
}

impl Content {
  pub fn get_se(&self) -> &ScreenshotExecutor {
    &self.screenshot_executor
  }
	pub fn set_win_type(&mut self, t: WindowType) {
		self.window_type = t;
	}
  pub fn current_screen(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let di = self.get_current_screen_di(_frame).unwrap();
    match self.get_se().screenshot(di,  None) {
      Ok(screenshot) => {
        let img_bytes = screenshot.rgba().clone();
        let img_bytes_fast = screenshot.to_png(None).unwrap();
        ctx.memory_mut(|mem|{
          mem.data.insert_temp(Id::from("screenshot"), img_bytes);
          mem.data.insert_temp(Id::from("bytes"), img_bytes_fast.clone());
          mem.data.insert_temp(Id::from("width"), screenshot.width());
          mem.data.insert_temp(Id::from("height"), screenshot.height());
        });
        self.set_win_type(Screenshot);
      }
      Err(error) => {
        error!("{}" , error);
      }
    }
  }
  
  pub fn select(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    let di = self.get_current_screen_di(_frame);
    if di.is_some(){
      match self.get_se().screenshot(di.unwrap(),None) {
        Ok(screenshot) => {
          let img_bytes = screenshot.rgba().clone();
          let img_bytes_fast = screenshot.to_png(None).unwrap();
          ctx.memory_mut(|mem|{
            mem.data.insert_temp(Id::from("screenshot"), img_bytes);
            mem.data.insert_temp(Id::from("bytes"), img_bytes_fast.clone());
            mem.data.insert_temp(Id::from("width"), screenshot.width());
            mem.data.insert_temp(Id::from("height"), screenshot.height());
            mem.data.insert_temp(Id::from("di"), di.unwrap());
          });
          self.set_win_type(Select);
        }
        Err(error) => {
          error!("{}",error);
        }
      }
    }
    else {
      println!("Errore nella selezione dello schermo.");
    }
  }
  
  pub fn all_screens(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
    
    let images = match self.get_se().screenshot_all() {
      None => {
        error!("Error during screens acquisition.");
        return;
      }
      Some(images) => {images}
    };
    
    let screenshot = ImageCombiner::combine(images).unwrap();
    let img_bytes = screenshot.rgba().clone();
    let img_bytes_fast = screenshot.to_png(None).unwrap();
    ctx.memory_mut(|mem|{
      mem.data.insert_temp(Id::from("screenshot"), img_bytes);
      mem.data.insert_temp(Id::from("bytes"), img_bytes_fast.clone());
      mem.data.insert_temp(Id::from("width"), screenshot.width());
      mem.data.insert_temp(Id::from("height"), screenshot.height());
    });
    self.set_win_type(Screenshot);
  }
  
  pub fn get_current_screen_di(&mut self, _frame: &mut eframe::Frame) -> Option<DisplayInfo> {
    match Mouse::get_mouse_position() {
      Mouse::Position { x, y } => DisplayInfo::from_point(x, y).ok(),
      Mouse::Error => panic!("Error in mouse position"),
    }
  }
  
  pub fn save_image(&mut self, ctx: &Context) {
    let mut image = screenshots::Image::new(0,0,vec![]);
    ctx.memory(|mem|{
      let image_bytes = mem.data.get_temp::<Vec<u8>>(Id::from("screenshot"));
      if image_bytes.is_some(){
        let image_width = mem.data.get_temp::<u32>(Id::from("width")).unwrap().clone();
        let image_height = mem.data.get_temp::<u32>(Id::from("height")).unwrap().clone();
        image = screenshots::Image::new(image_width, image_height, image_bytes.clone().unwrap());
      }
      let imgf = ImageFormatter::from(image);
      imgf.save_fmt("target/screenshot".to_string(), ImageFmt::PNG);
    });
  }
  
  pub fn copy_image(&mut self, ctx: &Context) {
    let mut image = screenshots::Image::new(0,0,vec![]);
    ctx.memory(|mem|{
      let image_bytes = mem.data.get_temp::<Vec<u8>>(Id::from("screenshot"));
      if image_bytes.is_some(){
        let image_width = mem.data.get_temp::<u32>(Id::from("width")).unwrap().clone();
        let image_height = mem.data.get_temp::<u32>(Id::from("height")).unwrap().clone();
        image = screenshots::Image::new(image_width, image_height, image_bytes.clone().unwrap());
      }
      let imgf = ImageFormatter::from(image);
      imgf.to_clipboard();
    });
  }
  
}

impl eframe::App for Content {
  
  fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
    match self.window_type{
      Main => self.main_window(ctx, _frame),
      Settings => self.settings_window(ctx, _frame),
      Screenshot => self.screenshot_window(ctx, _frame),
      Select => self.select_window(ctx, _frame),
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
    initial_window_size: Some(Vec2::new(330.0, 230.0)),
    default_theme: Theme::Dark,
    // centered: true,
    ..Default::default()
  };

  let content = Content{
    configuration,
    screenshot_executor: s,
    encoders,
    text: "".to_string(),
    window_type: Settings,
    region: None,
  };

  run_native(
    &*app_name_tmp,
    options,
    Box::new(move |_cc| Box::<Content>::new(content)),
  ).expect("Error during gui thread init.");
}
