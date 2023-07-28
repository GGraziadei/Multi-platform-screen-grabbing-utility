use std::cell::{Cell, RefCell};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;
use egui::Key;
use image::ImageFormat;
use serde::{Deserialize, Serialize};


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

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration{
    app_name : RefCell<String>,
    save_path : RefCell<String>,
    image_format : RefCell<ImageFmt>,
    coordinates : Cell<(usize, usize)>,
    height: Cell<usize>,
    width: Cell<usize>,
    delay: Cell<Option<Duration>>,
    acquire_mode : Cell<AcquireMode>,
    hot_key : Cell<KeyCombo>
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
        image_format : ImageFmt,
        coordinates : (usize, usize),
        height: usize,
        width: usize,
        delay: Option<Duration>,
        acquire_mode : AcquireMode,
        hot_key : KeyCombo
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
            hot_key: Cell::new(hot_key),
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

    pub fn get_hot_key(&self) -> Option<Vec<Key>>
    {
        let val = self.hot_key.get();
        let mut res = Vec::<Key>::new();

        if let Some(k1) = val.k1{
            res.push(k1);
        }

        if let Some(k2) = val.k2{
            res.push(k2);
        }

        if let Some(k3) = val.k3{
            res.push(k3);
        }

        Some(res)

    }

    pub fn set_hot_key(&self, hot_key : KeyCombo ) -> Option<bool>
    {
        self.hot_key.set(hot_key);
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