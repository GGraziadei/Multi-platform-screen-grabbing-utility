use std::fmt::Error;
use std::fs::File;
use std::path::Path;
use image::{ColorType, Frame, ImageFormat, ImageResult};
use image::ColorType::Rgba8;
use image::ImageFormat::Png;
use screenshots::Image;
use crate::configuration::ImageFmt;
use crate::configuration::ImageFmt::GIF;

pub struct ImageFormatter {
    buffer : Vec<u8>,
    width : u32,
    height : u32,
    color_type : ColorType
}

impl From<Image> for ImageFormatter {
    fn from(value: Image) -> Self {
        Self{
            buffer: value.rgba().clone(),
            width: value.width(),
            height: value.height(),
            color_type: ColorType::Rgba8
        }
    }
}

impl ImageFormatter{
    pub fn save_fmt(&self, path : String, fmt : ImageFmt ) -> Option<()>
    {
        let image_format = fmt.get_image_format()?;
        let mut p : String = String::from(path);
        p.push_str(&fmt.get_image_ext()?);

        image::save_buffer_with_format(Path::new(&p),&self.buffer,self.width,self.height,self.color_type,Png).ok()
    }
}