use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use image::ImageFormat;
use serde::{Deserialize, Serialize, Serializer};
use serde::de::Unexpected::Str;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum AcquireMode{
    Window,
    DragDrop
}

impl Default for AcquireMode{
    fn default() -> Self {
        AcquireMode::DragDrop
    }
}

#[derive(Serialize, Deserialize, Copy, Clone,PartialEq)]
pub enum ImageFmt{
    PNG,
    JPG,
    GIF
}

impl Default for ImageFmt{
    fn default() -> Self {
        ImageFmt::PNG
    }
}

impl Display for ImageFmt{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFmt::PNG => {write!(f, "PNG")}
            ImageFmt::JPG => {write!(f, "JPEG")}
            ImageFmt::GIF => {write!(f, "GIF")}
        }

    }
}

impl ImageFmt{

    pub fn get_image_format(&self) -> Option<ImageFormat>
    {
        Some(match self{
            ImageFmt::PNG => {ImageFormat::Png}
            ImageFmt::JPG => {ImageFormat::Jpeg}
            ImageFmt::GIF => {ImageFormat::Gif}
        })
    }

    pub fn get_image_ext(&self) -> Option<String>
    {
        Some(match self{
            ImageFmt::PNG => {".png".to_string()}
            ImageFmt::JPG => {".jpeg".to_string()}
            ImageFmt::GIF => {".gif".to_string()}
        })
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration{
    app_name : RefCell<String>,
    save_path : RefCell<String>,
    image_format : RefCell<ImageFmt>,
    coordinates : Cell<(usize, usize)>,
    height: Cell<usize>,
    width: Cell<usize>,
    delay: Cell<Option<Duration>>,
    acquire_mode : Cell<AcquireMode>
}

const SETTINGS_FILE: &'static str = "settings.json";


impl Configuration{

    /*
        Se presente un file di configurazione lo deserializza
        altrimenti crea un file di configurazione di default e lo serializza
    */
    pub fn new() -> Self{

        match Self::read() {
            None => {
                let c = Self::default();
                c.write();
                c
            }
            Some(c) => { c }
        }
    }

    pub fn bulk(
        app_name : String,
        save_path : String,
        image_format : ImageFmt,
        coordinates : (usize, usize),
        height: usize,
        width: usize,
        delay: Option<Duration>,
        acquire_mode : AcquireMode
    ) -> Self
    {
        let c = Self{
            app_name: RefCell::new(app_name),
            save_path: RefCell::new(save_path),
            image_format: RefCell::new(image_format),
            coordinates: Cell::new(coordinates),
            height: Cell::new(height),
            width: Cell::new(width),
            delay: Cell::new(delay),
            acquire_mode: Cell::new(acquire_mode),
        };
        c.write().expect("Error during config file generation.");
        c
    }

    pub fn get_app_name(&self) -> Option<String>{
       Some((self.app_name.try_borrow().ok()?).clone())
    }

    pub fn set_app_name(&self , app_name : String) -> Option<bool>
    {
        *self.app_name.borrow_mut() =app_name;
        self.write()?;
        Some(true)
    }

    pub fn get_save_path(&self) -> Option<String> {
        let s = self.save_path.borrow().clone();
        Some(s)
    }

    pub fn set_save_path(&self , save_path : String) -> Option<bool>
    {
        *self.save_path.borrow_mut() =save_path;
        self.write()?;
        Some(true)
    }

    pub fn get_image_fmt(&self) -> Option<ImageFmt> {
        let s = self.image_format.borrow().clone();
        Some(s)
    }

    pub fn set_image_fmt(&self , image_format : ImageFmt) -> Option<bool>
    {
        *self.image_format.borrow_mut() =image_format;
        self.write()?;
        Some(true)
    }

    pub fn get_coordinates(&self) -> Option<(usize, usize)>
    {
        Some(self.coordinates.get())
    }

    pub fn set_coordinates(&self, coordinates : (usize, usize)) -> Option<bool>
    {
        self.coordinates.set(coordinates);
        self.write()?;
        Some(true)
    }

    pub fn get_height(&self) -> Option<usize>
    {
        Some(self.height.get())
    }

    pub fn set_height(&self, height : usize ) -> Option<bool>
    {
        self.height.set(height);
        self.write()?;
        Some(true)
    }

    pub fn get_width(&self)  -> Option<usize>
    {
        Some(self.width.get())
    }

    pub fn set_width(&self, width : usize ) -> Option<bool>
    {
        self.width.set(width);
        self.write()?;
        Some(true)
    }

    pub fn get_acquire_mode(&self) -> Option<AcquireMode>
    {
        Some(self.acquire_mode.get())
    }

    pub fn set_acquire_mode(&self, acquire_mode : AcquireMode ) -> Option<bool>
    {
        self.acquire_mode.set(acquire_mode);
        self.write()?;
        Some(true)
    }

    pub fn get_delay(&self) -> Option<Duration>
    {
        self.delay.get()
    }

    pub fn set_delay(&self, delay : Option<Duration> ) -> Option<bool>
    {
        self.delay.set(delay);
        self.write()?;
        Some(true)
    }

    fn write(&self) -> Option<&'static str>
    {
        let mut serialized =  serde_json::to_string(self).ok()?;
        let mut file = File::create(SETTINGS_FILE).ok()?;
        file.write(serialized.as_ref()).ok()?;

        Some(SETTINGS_FILE)
    }

    fn read() -> Option<Self>
    {
        let mut buf = String::new();
        let mut file = File::open(SETTINGS_FILE).ok()?;
        file.read_to_string(&mut buf).ok()?;
        let deserialized : Self = serde_json::from_str(&buf ).ok()?;

        Some(deserialized)
    }
}