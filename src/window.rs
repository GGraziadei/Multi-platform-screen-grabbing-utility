use std::default::Default;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use eframe::{egui, run_native, Theme};
use egui::*;
use log::error;
use mouse_position::mouse_position::Mouse;
use screenshots::DisplayInfo;
use crate::configuration::{AcquireMode, Configuration, ImageFmt};
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshots::{ScreenshotExecutor};
use WindowType::*;
use crate::image_combiner::ImageCombiner;

pub enum WindowType {
  Main,
  Settings,
  Preview,
  SelectScreen,
  DragAndDrop
}

#[derive(Clone)]
pub struct GuiScreenshotData {
  pub(crate)img_bytes: Vec<u8>,
  pub(crate)img_bytes_fast: Vec<u8>,
  pub(crate)width: u32,
  pub(crate)height: u32
}

pub struct Content {
  pub(crate) configuration : Arc<RwLock<Configuration>>,
  screenshot_executor : ScreenshotExecutor,
  encoders : Arc<Mutex<Vec<EncoderThread>>>,
  text: String,
  window_type: WindowType,
  region: Option<Rect>,
  colorimage: Option<ColorImage>,
  acquire: Option<AcquireMode>,
  gui_screenshot_data: Option<GuiScreenshotData>,
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
  pub fn get_region(&self) -> Option<Rect> {
    self.region
  }
  pub fn set_region(&mut self, region: Rect) {
    self.region = Some(region)
  }
  pub fn get_colorimage(&self) -> Option<ColorImage> {
    self.colorimage.clone()
  }
  pub fn set_colorimage(&mut self, image: ColorImage) {
    self.colorimage = Some(image)
  }
  pub fn get_acquire(&self) -> Option<AcquireMode> {
    self.acquire
}
  pub fn set_acquire(&mut self, acquire: Option<AcquireMode>) {
    self.acquire = acquire
  }
  pub fn get_gui_screenshot_data(&self) -> Option<GuiScreenshotData> {
    self.gui_screenshot_data.clone()
  }
  pub fn set_gui_screenshot_data(&mut self, data: Option<GuiScreenshotData>) {
    self.gui_screenshot_data = data
  }
  
  pub fn current_screen(&mut self, _frame: &eframe::Frame){
    let di = self.get_current_screen_di(_frame).unwrap();
    match self.get_se().screenshot(di,  None) {
      Ok(screenshot) => {
        let img_bytes = screenshot.rgba().clone();
        let img_bytes_fast = screenshot.to_png(None).unwrap();
        // ctx.memory_mut(|mem|{
        //   mem.data.insert_temp(Id::from("screenshot"), img_bytes);
        //   mem.data.insert_temp(Id::from("bytes"), img_bytes_fast.clone());
        //   mem.data.insert_temp(Id::from("width"), screenshot.width());
        //   mem.data.insert_temp(Id::from("height"), screenshot.height());
        // });
        self.set_gui_screenshot_data(Some(GuiScreenshotData{
          img_bytes,
          img_bytes_fast,
          width: screenshot.width(),
          height: screenshot.height()
        }));
        self.set_win_type(Preview);
      }
      Err(error) => {
        error!("{}" , error);
      }
    }
    
    self.colorimage = None;
  }
  pub fn select_screen(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
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
        self.set_win_type(SelectScreen);
      }
      Err(error) => {
        error!("{}" , error);
      }
    }

    self.colorimage = None;
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
          });
          self.set_win_type(DragAndDrop);
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
    self.set_win_type(Preview);
    self.colorimage = None;
  }
  
  pub fn get_current_screen_di(&mut self, _frame: &eframe::Frame) -> Option<DisplayInfo> {
    match Mouse::get_mouse_position() {
      Mouse::Position { x, y } => DisplayInfo::from_point(x, y).ok(),
      Mouse::Error => panic!("Error in mouse position"),
    }
  }
  
  pub fn save_image(&mut self, ctx: &Context, custom_path: Option<PathBuf>) {

    let mut path : String;
    let mut format : ImageFmt;

    if custom_path.is_some(){
      match custom_path.clone().unwrap().extension(){
        Some(ext) => {
          match ext.to_str().unwrap() {
            "png" => format = ImageFmt::PNG,
            "jpg" => format = ImageFmt::JPG,
            "jpeg" => format = ImageFmt::JPG,
            "gif" => format = ImageFmt::GIF,
            _ => {
              error!("Format not supported.");
              return;
            }
          }
        }
        None => {
          error!("Format not supported.");
          return;
        }
      }
      path = custom_path.unwrap().to_str().unwrap().to_string();
    }
    else
    {
      let configuration_read = self.configuration.read().unwrap();
      let file_name = configuration_read.get_filename().unwrap();
      let save_path = configuration_read.get_save_path().unwrap();
      format = configuration_read.get_image_fmt().unwrap();
      drop(configuration_read);

      path = Path::new(&save_path).join(format!("{}.{}",file_name,format)).to_str().unwrap().to_string();
      println!("{}",path);
    }

    let imgf = match self.get_colorimage(){
      Some(img) => ImageFormatter::from(img),
      None => {
        let mut image = screenshots::Image::new(0,0,vec![]);
        let imgf = ctx.memory(|mem|{
          let image_bytes = mem.data.get_temp::<Vec<u8>>(Id::from("screenshot"));
          if image_bytes.is_some(){
            let image_width = mem.data.get_temp::<u32>(Id::from("width")).unwrap().clone();
            let image_height = mem.data.get_temp::<u32>(Id::from("height")).unwrap().clone();
            image = screenshots::Image::new(image_width, image_height, image_bytes.clone().unwrap());
          }
          ImageFormatter::from(image)
        });
        imgf
      }
    };
    imgf.save_fmt(path, format);
  }
  
  pub fn copy_image(&mut self, ctx: &Context) {
    if self.get_colorimage().is_some(){
      ImageFormatter::from(self.get_colorimage().unwrap()).to_clipboard();
    }
    else {
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
  
}

impl eframe::App for Content {

  fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {
    if self.region.is_some(){
      // let colorimage = _frame.screenshot().unwrap().region(&self.region.unwrap(), None);
      let mut region = self.region.unwrap();
      let mut colorimage = _frame.screenshot().unwrap(); //.region(&Rect::from_min_max(pos2(0.0, 0.0), pos2(1920.0, 1080.0)), None);
      region.min.x = (region.min.x*colorimage.size[0] as f32)/_frame.info().window_info.size.x;
      region.min.y = (region.min.y*colorimage.size[1] as f32)/_frame.info().window_info.size.y;
      region.max.x = (region.max.x*colorimage.size[0] as f32)/_frame.info().window_info.size.x;
      region.max.y = (region.max.y*colorimage.size[1] as f32)/_frame.info().window_info.size.y;
      colorimage = colorimage.region(&region, None);

      self.region = None;
      self.colorimage = Some(colorimage.clone());
      self.set_win_type(Preview);
    }
    else {
      match self.get_acquire(){
        Some(a) => {
          match a {
            AcquireMode::CurrentScreen => {self.current_screen(_frame)}
            AcquireMode::SelectScreen => {}
            AcquireMode::AllScreen => {}
            AcquireMode::DragDrop => {}
          }
        }
        None => {}
      }
    }
  }
  
  fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
     match self.get_acquire(){
      None => {}
      Some(acquire) => {
        match acquire {
          AcquireMode::CurrentScreen => {
            // self.current_screen(ctx, _frame);
            self.set_acquire(None);
            _frame.set_visible(true);
          }
          AcquireMode::SelectScreen => {
            // self.select_screen(ctx, _frame);
            self.set_acquire(None);
            _frame.set_visible(true);
          }
          AcquireMode::AllScreen => {
            // self.all_screens(ctx, _frame);
            self.set_acquire(None);
            _frame.set_visible(true);
          }
          AcquireMode::DragDrop => {
            // self.select(ctx, _frame);
            self.set_acquire(None);
            _frame.set_visible(true);
          }
        }
      }
    }
    match self.window_type{
      Main => self.main_window(ctx, _frame),
      Settings => self.settings_window(ctx, _frame),
      Preview => self.screenshot_window(ctx, _frame),
      SelectScreen => self.select_screen_window(ctx, _frame),
      DragAndDrop => self.select_window(ctx, _frame),
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
    default_theme: Theme::Dark,
    // centered: true,
    ..Default::default()
  };

  let content = Content{
    configuration,
    screenshot_executor: s,
    encoders,
    text: "".to_string(),
    window_type: Main,
    region: None,
    colorimage: None,
    acquire: None,
    gui_screenshot_data: None
  };

  run_native(
    &*app_name_tmp,
    options,
    Box::new(move |_cc| Box::<Content>::new(content)),
  ).expect("Error during gui thread init.");
}
