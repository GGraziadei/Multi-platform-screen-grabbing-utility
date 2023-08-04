use std::cell::{Cell, RefCell};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::Hash;
use std::io::{Read, Write};
use std::time::Duration;
use egui::epaint::ahash::HashMap;
use egui::Key;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use chrono::Local;


#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AcquireMode{
    /*Active window*/
    Window,
    /*Select screen*/
    Screen,
    /*Merge all screen*/
    AllScreen,
    /*Active screen (in front-end flag for edit image)*/
    DragDrop,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct AcquireAction{
    pub save_file : bool,
    pub copy_file : bool
}

impl Default for AcquireMode{
    fn default() -> Self {
        AcquireMode::DragDrop
    }
}

impl Display for AcquireMode{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AcquireMode::Window => { "Window" }
            AcquireMode::Screen => { "Active screen" }
            AcquireMode::AllScreen => { "All screens" }
            AcquireMode::DragDrop => { "Drag and drop" }
        })
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

#[derive(Serialize, Deserialize, Copy, Clone,PartialEq)]
pub struct KeyCombo{
    k1 : Option<Key>,
    k2 : Option<Key>,
    k3 : Option<Key>
}

impl Default for KeyCombo{
    fn default() -> Self {
        Self{
            k1: None,
            k2: None,
            k3: None,
        }
    }
}

impl KeyCombo{

    pub fn new(combo : Vec<Key>) -> Self
    {
        assert!(combo.len()<=3 && combo.len()>0);

        let mut k1 = None;
        let mut k2 = None;
        let mut k3 = None;

        if combo.len() >= 1 {
            k1 = Some(combo[0]);
        }

        if combo.len() >= 2 {
            k1 = Some(combo[0]);
        }

        if combo.len() == 3 {
            k1 = Some(combo[0]);
        }

        Self{
            k1,
            k2,
            k3,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration{
    app_name : String,
    save_path : String,
    filename_pattern : String,
    image_format : ImageFmt,
    coordinates : (usize,usize),
    height: usize,
    width: usize,
    when_capture : AcquireAction,
    delay: Option<Duration>,
    hot_key_map : HashMap<AcquireMode, KeyCombo>
}

impl Default for Configuration{
    fn default() -> Self {
        Self{
            app_name: "MPSGU".to_string(),
            save_path: "".to_string(),
            filename_pattern: "Screenshot_%Y-%m-%d_%H:%M:%S".to_string(),
            image_format: ImageFmt::PNG,
            coordinates: (0, 0),
            height: 0,
            width: 0,
            when_capture: Default::default(),
            delay: None,
            hot_key_map: Default::default(),
        }
    }
}

const SETTINGS_FILE: &'static str = "settings.json";

impl Configuration{

    /*
        Create settings.json if absent open if present.
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
        filename_pattern : String,
        image_format : ImageFmt,
        coordinates : (usize, usize),
        height: usize,
        width: usize,
        delay: Option<Duration>,
        when_capture : AcquireAction,
        hot_key_map : HashMap<AcquireMode, KeyCombo>
    ) -> Self
    {
        let c = Self{
            app_name,
            save_path,
            filename_pattern,
            image_format,
            coordinates,
            height,
            width,
            when_capture,
            delay,
            hot_key_map,
        };
        c.write().expect("Error during config file generation.");
        c
    }

    pub fn get_app_name(&self) -> Option<String>
    {
       Some((self.app_name).clone())
    }

    pub fn set_app_name(&mut self , app_name : String) -> Option<bool>
    {
        self.app_name = app_name;
        self.write()?;
        Some(true)
    }

    pub fn get_save_path(&self) -> Option<String>
    {
        Some(self.save_path.clone())
    }

    pub fn set_save_path(&mut self , save_path : String) -> Option<bool>
    {
        self.save_path=save_path;
        self.write()?;
        Some(true)
    }

    pub fn get_image_fmt(&self) -> Option<ImageFmt>
    {
        Some(self.image_format.clone())
    }

    pub fn set_image_fmt(&mut self , image_format : ImageFmt) -> Option<bool>
    {
        self.image_format = image_format;
        self.write()?;
        Some(true)
    }

    pub fn get_coordinates(&self) -> Option<(usize, usize)>
    {
        Some(self.coordinates.clone())
    }

    pub fn set_coordinates(&mut self, coordinates : (usize, usize)) -> Option<bool>
    {
        self.coordinates = coordinates;
        self.write()?;
        Some(true)
    }

    pub fn get_height(&self) -> Option<usize>
    {
        Some(self.height)
    }

    pub fn set_height(&mut self, height : usize ) -> Option<bool>
    {
        self.height = height;
        self.write()?;
        Some(true)
    }

    pub fn get_width(&self)  -> Option<usize>
    {
        Some(self.width)
    }

    pub fn set_width(&mut self, width : usize ) -> Option<bool>
    {
        self.width = width;
        self.write()?;
        Some(true)
    }

    pub fn get_delay(&self) -> Option<Duration>
    {
        self.delay.clone()
    }

    pub fn set_delay(&mut self, delay : Option<Duration> ) -> Option<bool>
    {
        self.delay = delay;
        self.write()?;
        Some(true)
    }

    pub fn get_hot_key_map(&self) -> Option<HashMap<AcquireMode, KeyCombo>>
    {
        Some(self.hot_key_map.clone())
    }

    pub fn set_hot_key_map(&mut self, map: HashMap<AcquireMode, KeyCombo>) -> Option<bool>
    {
        self.hot_key_map = map;
        self.write()?;
        Some(true)
    }

    pub fn get_filename(&self) -> Option<String>
    {
        Some(chrono::Local::now().format(&self.filename_pattern.clone()).to_string())
    }

    pub fn set_filename_pattern(&mut self, p : String) -> Option<bool>
    {
        self.filename_pattern = p;
        self.write()?;
        Some(true)
    }

    pub  fn get_when_capture(&self) -> Option<AcquireAction>
    {
        Some(self.when_capture)
    }

    pub  fn set_when_capture(&mut self, aa : AcquireAction) -> Option<bool>
    {
        self.when_capture = aa;
        self.write()?;
        Some(true)
    }

    fn write(&self) -> Option<&'static str>
    {
        let serialized =  serde_json::to_string(self).ok()?;
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