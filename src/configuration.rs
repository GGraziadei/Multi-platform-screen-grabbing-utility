use std::collections::{HashMap};
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{Read, Write};
use std::time::Duration;
use egui::{Key, Modifiers, Rect};
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use directories::UserDirs;
use log::info;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum AcquireMode{
    /*Current screen*/
    CurrentScreen,
    /*Select screen*/
    SelectScreen,
    /*Merge all screen*/
    AllScreens,
    /*Portion of the current screen*/
    Portion,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct AcquireAction{
    pub save_file : bool,
    pub copy_file : bool
}

impl Default for AcquireMode{
    fn default() -> Self {
        AcquireMode::Portion
    }
}

impl Display for AcquireMode{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AcquireMode::CurrentScreen => { "Schermo corrente" }
            AcquireMode::SelectScreen => { "Seleziona schermo" }
            AcquireMode::AllScreens => { "Tutti gli schermi" }
            AcquireMode::Portion => { "Regione rettangolare" }
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

    pub fn get_image_format(&self) -> ImageFormat
    {
        match self{
            ImageFmt::PNG => {ImageFormat::Png}
            ImageFmt::JPG => {ImageFormat::Jpeg}
            ImageFmt::GIF => {ImageFormat::Gif}
        }
    }

}

#[derive(Serialize, Deserialize, Copy, Clone,PartialEq, Debug)]
pub struct KeyCombo{
    pub m: Modifiers,
    pub k: Option<Key>,
}

impl Default for KeyCombo{
    fn default() -> Self {
        Self{
            m: Modifiers::default(),
            k: None,
        }
    }
}

impl  Display for KeyCombo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut next = false;

        let command = self.m.clone();

        if command.alt {
            str.push_str("ALT");
            next = true;
        }

        if command.ctrl {
            if next {
                str.push('+');
            }
            str.push_str("CTRL");
            next = true;
        }

        if command.shift {
            if next {
                str.push('+');
            }
            str.push_str("SHIFT");
            next = true;
        }

        if command.mac_cmd {
            if next {
                str.push('+');
            }
            str.push_str("COMMAND");
            next = true;
        }


        if self.k.is_some(){
            if next {
                str.push('+');
            }
            let _str = format!("{:?}", self.k.clone().unwrap());
            str.push_str(_str.as_str());
        }
        
        write!(f, "{}",str)
    }
}

impl KeyCombo{

    pub fn new(modifiers : Modifiers, key: Option<Key>) -> Self
    {
        Self{
            m: modifiers,
            k: key,
        }
    }
    
    pub fn contains_key(&self) -> bool {
        self.k.is_some()
    }
    
}

#[derive(Serialize, Deserialize)]
pub struct Configuration{
    app_name : String,
    save_path : String,
    filename_pattern : String,
    image_format : ImageFmt,
    save_region: bool,
    region: Option<Rect>,
    when_capture : AcquireAction,
    delay: Option<Duration>,
    hot_key_map : HashMap<AcquireMode, KeyCombo>
}

impl Default for Configuration{
    fn default() -> Self {
        Self{
            app_name: "MPSGU".to_string(),
            save_path: UserDirs::new().unwrap().picture_dir().unwrap().to_str().unwrap().to_string(),
            filename_pattern: "Screenshot_%Y-%m-%d_%H%M%S".to_string(),
            image_format: ImageFmt::PNG,
            save_region: false,
            region: None,
            when_capture: Default::default(),
            delay: None,
            hot_key_map: HashMap::from([
                (AcquireMode::Portion, KeyCombo::new(Modifiers::default(), None)),
                (AcquireMode::AllScreens, KeyCombo::new(Modifiers::default(), None)),
                (AcquireMode::SelectScreen, KeyCombo::new(Modifiers::default(), None)),
                (AcquireMode::CurrentScreen, KeyCombo::new(Modifiers::default(), None))
            ]),
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
        &mut self,
        app_name : Option<String>,
        save_path : Option<String>,
        filename_pattern : Option<String>,
        image_format : Option<ImageFmt>,
        save_region: Option<bool>,
        region: Option<Option<Rect>>,
        delay: Option<Option<Duration>>,
        when_capture : Option<AcquireAction>,
        hot_key_map : Option<HashMap<AcquireMode, KeyCombo>>
    ) -> Option<bool>
    {

        if save_path.is_some() {
            //Check save path
            let save_path = save_path?;
            fs::read_dir(&save_path).ok()?;
            self.save_path = save_path;
        }

        if app_name.is_some(){
            self.app_name = app_name?;
        }

        if filename_pattern.is_some() {
            self.filename_pattern = filename_pattern?;
        }

        if image_format.is_some() {
            self.image_format = image_format?;
        }

        if save_region.is_some() {
            self.save_region = save_region?;
        }

        if region.is_some(){
            self.region = region?;
        }

        if delay.is_some() {
            self.delay = delay?;
        }

        if when_capture.is_some() {
            self.when_capture = when_capture?;
        }

        if hot_key_map.is_some() {
            self.hot_key_map = hot_key_map?;
        }

        self.write().expect("Error during config file generation.");
        Some(true)
    }

    pub fn get_app_name(&self) -> Option<String>
    {
       Some((self.app_name).clone())
    }

    /*
    pub fn set_app_name(&mut self , app_name : String) -> Option<bool>
    {
        self.app_name = app_name;
        self.write()?;
        Some(true)
    }*/

    pub fn get_save_path(&self) -> Option<String>
    {
        Some(self.save_path.clone())
    }

    /*
    pub fn set_save_path(&mut self , save_path : String) -> Option<bool>
    {
        self.save_path=save_path;
        self.write()?;
        Some(true)
    }*/

    pub fn get_image_fmt(&self) -> Option<ImageFmt>
    {
        Some(self.image_format.clone())
    }

    /*
    pub fn set_image_fmt(&mut self , image_format : ImageFmt) -> Option<bool>
    {
        self.image_format = image_format;
        self.write()?;
        Some(true)
    }*/

    pub fn get_save_region(&self) -> bool {
        self.save_region
    }

    /*
    pub fn set_save_region(&mut self, save_region: bool) -> Option<bool> {
        self.save_region = save_region;
        self.write()?;
        Some(true)
    }
     */

    pub fn get_region(&self) -> Option<Rect>
    {
        self.region.clone()
    }

    pub fn set_region(&mut self, region : Rect) -> Option<bool>
    {
        self.region = Some(region);
        self.write()?;
        Some(true)
    }

    pub fn get_delay(&self) -> Option<Duration>
    {
        self.delay.clone()
    }

    /*
    pub fn set_delay(&mut self, delay : Option<Duration> ) -> Option<bool>
    {
        self.delay = delay;
        self.write()?;
        Some(true)
    }*/

    pub fn get_hot_key_map(&self) -> Option<HashMap<AcquireMode, KeyCombo>>
    {
        Some(self.hot_key_map.clone())
    }

    /*
    pub fn set_hot_key_map(&mut self, map: HashMap<AcquireMode, KeyCombo>) -> Option<bool>
    {
        self.hot_key_map = map;
        self.write()?;
        Some(true)
    }

     */

    pub fn get_filename(&self) -> Option<String>
    {
        Some(chrono::Local::now().format(&self.filename_pattern.clone()).to_string())
    }

    pub fn get_filename_pattern(&self) -> Option<String>
    {
        Some(self.filename_pattern.clone())
    }

    /*
    pub fn set_filename_pattern(&mut self, p : String) -> Option<bool>
    {
        self.filename_pattern = p;
        self.write()?;
        Some(true)
    }

     */

    pub  fn get_when_capture(&self) -> Option<AcquireAction>
    {
        Some(self.when_capture)
    }

    /*
    pub  fn set_when_capture(&mut self, aa : AcquireAction) -> Option<bool>
    {
        self.when_capture = aa;
        self.write()?;
        Some(true)
    }
     */

    fn write(&self) -> Option<&'static str>
    {
        let serialized =  serde_json::to_string(self).ok()?;
        let mut file = File::create(SETTINGS_FILE).ok()?;
        file.write(serialized.as_ref()).ok()?;
        info!("settings updated");
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
