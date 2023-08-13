use std::collections::HashMap;
use std::default::Default;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use eframe::{egui, run_native, Theme};
use egui::*;
use log::error;
use mouse_position::mouse_position::Mouse;
use screenshots::DisplayInfo;
use crate::configuration::{AcquireMode, Configuration, ImageFmt, KeyCombo};
use crate::image_formatter::{EncoderThread, ImageFormatter};
use crate::screenshots::{ScreenshotExecutor};
use WindowType::*;
use crate::image_combiner::ImageCombiner;

#[derive(Eq, PartialEq)]
pub enum WindowType {
    Main,
    Settings,
    Preview,
    Portion,
    SelectScreen,
    Drawing
}

pub struct Content {
    pub(crate) configuration : Arc<RwLock<Configuration>>,
    screenshot_executor : ScreenshotExecutor,
    encoders : Arc<Mutex<Vec<EncoderThread>>>,
    window_type: WindowType,
    region: Option<Rect>,
    color_image: Option<ColorImage>,
    acquire_mode : Option<AcquireMode>
}

impl Content {
    pub fn get_se(&self) -> &ScreenshotExecutor {
        &self.screenshot_executor
    }
    pub fn set_win_type(&mut self, t: WindowType) {
        self.window_type = t;
    }
    pub fn set_region(&mut self, region: Rect) {
        self.region = Some(region)
    }
    pub fn get_color_image(&self) -> Option<ColorImage> {
        self.color_image.clone()
    }
    pub fn get_acquire_mode(&self) -> Option<AcquireMode> {self.acquire_mode}
    pub fn set_acquire_mode(&mut self, am : Option<AcquireMode>) -> () {self.acquire_mode = am}

    pub fn current_screen(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        match self.get_current_screen_di() {
            None => { error!("DisplayInfo not found"); }
            Some(di) => {
                match self.get_se().screenshot(di,  None) {
                    Ok(screenshot) => {
                        let img_bytes = screenshot.rgba().clone();
                        ctx.memory_mut(|mem|{
                            mem.data.insert_temp(Id::from("screenshot"), img_bytes);
                            if let Ok(img_bytes_fast) = screenshot.to_png(None){
                                mem.data.insert_temp(Id::from("bytes"), img_bytes_fast);
                            }
                            mem.data.insert_temp(Id::from("width"), screenshot.width());
                            mem.data.insert_temp(Id::from("height"), screenshot.height());
                        });
                        self.set_win_type(Preview);
                    }
                    Err(error) => { error!("{}" , error); }
                }
                self.color_image = None;
            }
        }
    }

    pub fn select_screen(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        match self.get_se().screenshot_all() {
            Some(v) => {
                let mut img_bytes_vec = vec![];
                let mut fast_bytes_vec = vec![];

                for i in v {
                    match i {
                        Ok(image) => {
                            let img_bytes = image.rgba().clone();
                            img_bytes_vec.push(img_bytes);
                            if let Ok( img_bytes_fast) = image.to_png(None){
                                fast_bytes_vec.push(img_bytes_fast);
                            }
                        }
                        Err(error) => {error!("{}",error); }
                    }
                }
                ctx.memory_mut(|mem|{
                    mem.data.insert_temp(Id::from("bytes"), img_bytes_vec);
                    mem.data.insert_temp(Id::from("r_bytes"), fast_bytes_vec.clone());
                });
                self.set_win_type(SelectScreen);

            }
            None => {}
        }

        self.color_image = None;
    }

    pub fn portion(&mut self, ctx: &Context, _frame: &mut eframe::Frame){
        match self.get_current_screen_di(){
            None => {
                error!("Error in selecting screen.");
            }
            Some(di) => {
                match self.get_se().screenshot(di,None) {
                    Ok(screenshot) => {
                        let img_bytes = screenshot.rgba().clone();
                        ctx.memory_mut(|mem|{
                            mem.data.insert_temp(Id::from("screenshot"), img_bytes);
                            if let Ok(img_bytes_fast) = screenshot.to_png(None){
                                mem.data.insert_temp(Id::from("bytes"), img_bytes_fast);
                            }
                            mem.data.insert_temp(Id::from("width"), screenshot.width());
                            mem.data.insert_temp(Id::from("height"), screenshot.height());
                            mem.data.insert_temp(Id::from("di"), di);
                        });
                        self.set_win_type(Portion);
                    }
                    Err(error) => {
                        error!("{}",error);
                    }
                }
            }
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

        match ImageCombiner::combine(images) {
            None => {
                error!("Error during image combination.");
                notifica::notify("Error in screenshot acquisition.", "")
                    .expect("OS API error.");
                return;
            }
            Some(screenshot) => {
                let img_bytes = screenshot.rgba().clone();
                ctx.memory_mut(|mem|{
                    mem.data.insert_temp(Id::from("screenshot"), img_bytes);
                    if let Ok(img_bytes_fast) = screenshot.to_png(None){
                        mem.data.insert_temp(Id::from("bytes"), img_bytes_fast);
                    }
                    mem.data.insert_temp(Id::from("width"), screenshot.width());
                    mem.data.insert_temp(Id::from("height"), screenshot.height());
                });
                self.set_win_type(Preview);
                self.color_image = None;
            }
        }
    }

    pub fn get_current_screen_di(&mut self) -> Option<DisplayInfo> {
        match Mouse::get_mouse_position() {
            Mouse::Position { mut x, mut y } => {
                for display in DisplayInfo::all()
                    .expect("Error in screen list access")
                {
                    let new_x = (x as f32/display.scale_factor) as i32;
                    let new_y = (y as f32/display.scale_factor) as i32;
                    if new_x > display.x && new_x < display.x + display.width as i32 && new_y > display.y && new_y < display.y + display.height as i32 {
                        x = new_x;
                        y = new_y;
                        break;
                    }
                }
                DisplayInfo::from_point(x, y).ok()
            },
            Mouse::Error => panic!("Error in mouse position"),
        }
    }

    pub fn save_image(&mut self, ctx: &Context, custom_path: Option<PathBuf>) {

        let path : String;
        let format : ImageFmt;

        match custom_path {
            None => {
                let configuration_read = self.configuration.read().unwrap();
                let file_name = configuration_read.get_filename().unwrap();
                let save_path = configuration_read.get_save_path().unwrap();
                format = configuration_read.get_image_fmt().unwrap();
                drop(configuration_read);

                path = Path::new(&save_path).join(format!("{}.{}",file_name,format)).to_str().unwrap().to_string();
            },
            Some(custom_path) => {
                match custom_path.clone().extension(){
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
                path = custom_path.to_str().unwrap().to_string();
            }
        }

        let imgf = match self.get_color_image(){
            Some(img) => ImageFormatter::from(img),
            None => {
                ctx.memory(|mem|{
                    ImageFormatter::from(match mem.data.get_temp::<Vec<u8>>(Id::from("screenshot")){
                        None => {
                            screenshots::Image::new(0,0,vec![])
                        }
                        Some(image_bytes) => {
                            let image_width = mem.data.get_temp::<u32>(Id::from("width")).unwrap().clone();
                            let image_height = mem.data.get_temp::<u32>(Id::from("height")).unwrap().clone();
                            screenshots::Image::new(image_width, image_height, image_bytes.clone())
                        }
                    })
                })
            }
        };
        let mut encoders = self.encoders.lock()
            .expect("Error in encoders access");
        encoders.push(imgf.save_fmt(path, format));
        drop(encoders);
    }

    pub fn copy_image(&mut self, ctx: &Context) {
        match self.get_color_image(){
            None => {
                ctx.memory(|mem|{
                    match mem.data.get_temp::<Vec<u8>>(Id::from("screenshot")){
                        None => {}
                        Some(image_bytes) => {
                            let image_width = mem.data.get_temp::<u32>(Id::from("width")).unwrap().clone();
                            let image_height = mem.data.get_temp::<u32>(Id::from("height")).unwrap().clone();
                            ImageFormatter::from(screenshots::Image::new(image_width, image_height, image_bytes.clone()))
                                .to_clipboard()
                                .expect("Error in image processing. Panic on main thread.");
                        }
                    }
                });
            }
            Some(color_image) => {
                ImageFormatter::from(color_image).to_clipboard()
                    .expect("Error in image processing. Panic Main thread.");
            }
        }
    }

}

impl eframe::App for Content {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        match self.get_acquire_mode() {
            None => {}
            Some(am) => {
                _frame.set_visible(true);
                self.set_acquire_mode(None);

                match am {
                    AcquireMode::CurrentScreen => {
                        self.current_screen(ctx, _frame);
                    }
                    AcquireMode::SelectScreen => {
                        self.select_screen(ctx, _frame);
                    }
                    AcquireMode::AllScreens => {
                        self.all_screens(ctx, _frame);
                    }
                    AcquireMode::Portion => {
                        self.portion(ctx, _frame);
                    }
                }

                return;
            }
        }

        match self.window_type{
            Main => self.main_window(ctx, _frame),
            Settings => self.settings_window(ctx, _frame),
            Preview => self.screenshot_window(ctx, _frame),
            Portion => self.select_window(ctx, _frame),
            SelectScreen => self.select_screen_window(ctx, _frame),
            Drawing => self.drawing_window(ctx, _frame),
        }

        /*
            Shortcut :
                - Preview
                - Main
                - Select screen
        */
        if self.window_type != Settings
            && self.window_type != Drawing
            && self.window_type != Portion
        {
            ctx.input(|i| {
                let hkm = match ctx.memory(|mem| mem.data.get_temp::<HashMap<AcquireMode, KeyCombo>>(Id::from("hot_key_map"))) {
                    Some(hkm) => hkm,
                    None => match self.configuration.read(){
                        Ok(config) => {
                            if let Some(hkm) = config.get_hot_key_map(){
                                hkm
                            }else{
                                panic!("Error in keyboard shortcut map access.");
                            }
                        }
                        Err(error) => {
                            /*Gui thread have to access to configuration file. If it is poisoned panic*/
                            panic!("{}", error);
                        }
                    }
                };

                for (am, kc) in hkm {
                    if let Some(k) = kc.k {
                        if i.clone().consume_shortcut(&KeyboardShortcut::new(kc.m, k)){
                            _frame.set_visible(false);
                            match am {
                                AcquireMode::CurrentScreen => {
                                    self.set_acquire_mode(Some(AcquireMode::CurrentScreen));
                                }
                                AcquireMode::SelectScreen => {
                                    self.set_acquire_mode(Some(AcquireMode::SelectScreen));
                                }
                                AcquireMode::AllScreens => {
                                    self.set_acquire_mode(Some(AcquireMode::AllScreens));
                                }
                                AcquireMode::Portion => {
                                    self.set_acquire_mode(Some(AcquireMode::Portion));
                                }
                            }
                            return;
                        }
                    }
                }
            });
        }
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {

        if let Some(mut region) = self.region {
            // let colorimage = _frame.screenshot().unwrap().region(&self.region.unwrap(), None);
            //let mut region = self.region.unwrap();

            let mut colorimage = match _frame.screenshot(){
                None => {
                    notifica::notify("Error in screenshot acquisition.", "")
                        .expect("OS API error.");
                    self.set_win_type(Preview);
                    return;
                }
                Some(s) => {s}
            }; //.region(&Rect::from_min_max(pos2(0.0, 0.0), pos2(1920.0, 1080.0)), None);
            region.min.x = (region.min.x*colorimage.size[0] as f32)/_frame.info().window_info.size.x;
            region.min.y = (region.min.y*colorimage.size[1] as f32)/_frame.info().window_info.size.y;
            region.max.x = (region.max.x*colorimage.size[0] as f32)/_frame.info().window_info.size.x;
            region.max.y = (region.max.y*colorimage.size[1] as f32)/_frame.info().window_info.size.y;
            colorimage = colorimage.region(&region, None);

            self.region = None;
            self.color_image = Some(colorimage.clone());
            self.set_win_type(Preview);
        }
    }
}

pub fn draw_window(configuration: Arc<RwLock<Configuration>>, encoders: Arc<Mutex<Vec<EncoderThread>>>, s : ScreenshotExecutor){

    let app_name = match configuration.read(){
        Ok(config) => {
            match config.get_app_name() {
                None => {
                    panic!("Configuration error: app name is required.")
                }
                Some(app_name) => {app_name}
            }
        }
        Err(error) => {
            panic!("{}" ,error)
        }
    };

    let options = eframe::NativeOptions{
        resizable: true,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        // centered: true,
        ..Default::default()
    };

    let content = Content{
        configuration,
        screenshot_executor: s,
        encoders,
        window_type: Main,
        region: None,
        color_image: None,
        acquire_mode: None,
    };

    run_native(
        &app_name,
        options,
        Box::new(move |_cc| Box::<Content>::new(content)),
    ).expect("Error during gui thread init.");
}
