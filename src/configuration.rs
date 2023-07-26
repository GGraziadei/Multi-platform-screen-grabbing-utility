use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration{
    app_name : RefCell<String>,
    save_path : RefCell<String>,
    file_ext : RefCell<String>,
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
        file_ext : String,
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
            file_ext: RefCell::new(file_ext),
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

    pub fn get_file_ext(&self) -> Option<String> {
        let s = self.file_ext.borrow().clone();
        Some(s)
    }

    pub fn set_file_ext(&self , file_ext : String) -> Option<bool>
    {
        *self.file_ext.borrow_mut() =file_ext;
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