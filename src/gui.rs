use std::sync::{Arc, Mutex, RwLock};
use crate::configuration::{Configuration};
use crate::image_formatter::{EncoderThread};
use crate::draw_window::draw_window;
use crate::screenshots::{ScreenshotExecutor};


/*Marker struct. The event loop is executed over the main thread.*/
pub struct GuiThread;

impl GuiThread {
  pub fn new(configuration: Arc<RwLock<Configuration>>, encoders: Arc<Mutex<Vec<EncoderThread>>>, s : ScreenshotExecutor) -> Self  {
    draw_window(configuration, encoders, s);
    Self
  }
}

