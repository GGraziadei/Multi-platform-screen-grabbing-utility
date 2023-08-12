use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::thread::{JoinHandle,  spawn};
use arboard::{Clipboard, ImageData};
use egui::ColorImage;
use image::{ColorType, ImageError, ImageFormat, ImageResult};
use image::ColorType::Rgba8;
use image::error::{EncodingError, ImageFormatHint};
use image::ImageFormat::{Jpeg, Png, Gif};
use log::{info};
use screenshots::Image;
use crate::configuration::ImageFmt;

#[derive(Debug)]
pub struct EncoderThread{
    pub thread : JoinHandle<ImageResult<()>>,
    image_name : String
}

impl EncoderThread{

    pub fn get_image_name(&self) -> &String
    {
        &self.image_name
    }

}

#[derive(Clone)]
pub struct ImageFormatter {
    pub buffer : Vec<u8>,
    width : u32,
    height : u32,
    color_type : ColorType
}

impl From<Image> for ImageFormatter {
    fn from(value: Image) -> Self
    {
        Self{
            buffer: value.rgba().clone(),
            width: value.width(),
            height: value.height(),
            color_type: Rgba8
        }
    }
}

impl From<ColorImage> for ImageFormatter {
    fn from(value: ColorImage) -> Self
    {
        Self{
            buffer: Vec::from(value.as_raw().clone()),
            width: value.width() as u32,
            height: value.height() as u32,
            color_type: Rgba8,
        }
    }
}

const JPEG_QUALITY: u8 = 85;

impl ImageFormatter{

    fn encoder_thread(formatter : ImageFormatter, path : String, format : ImageFormat) -> ImageResult<()>
    {
        /*Screenshots crate produces image in PNG format. this thread encodes
         image in final format  with image crate */
        let p = Path::new(&path);

        match format {
            Png => {
                info!("PNG encoding");
                let result = image::save_buffer_with_format(p,&formatter.buffer,formatter.width,formatter.height,formatter.color_type,Png);
                info!("PNG encoding end.");
                notifica::notify("PNG encoding end.", format!("PNG encoding end. File available: {}", path.as_str()).as_str())
                    .expect("OS API error.");
                result
            }
            Jpeg => {
                info!("JPEG encoding");
                let w = File::create(p)?;
                let w_buffer = BufWriter::with_capacity(formatter.buffer.len(), w);
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(w_buffer, JPEG_QUALITY);
                let result = encoder.encode(&formatter.buffer, formatter.width, formatter.height, formatter.color_type);
                info!("JPEG encoding end.");
                notifica::notify("JPEG encoding end.", format!("JPEG encoding end. File available: {}", path.as_str()).as_str())
                    .expect("OS API error.");
                result
            }
            Gif => {
                info!("GIF encoding");
                let w = File::create(p)?;
                let w_buffer = BufWriter::with_capacity(formatter.buffer.len(), w);
                /*Only 1 frame for this reason the speed is 1*/
                let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(w_buffer,10);
                /*
                if frames number > 1
                let rgba_image = image::load_from_memory(&formatter.buffer)?.to_rgba8();
                let frame = image::Frame::new(rgba_image);
                */
                let result = encoder.encode(&formatter.buffer, formatter.width, formatter.height, formatter.color_type);
                info!("GIF encoding end.");
                notifica::notify("GIF encoding end.", format!("GIF encoding end. File available: {}", path.as_str()).as_str())
                    .expect("OS API error.");
                result
            }
            /*
            ImageFormat::WebP => {}
            ImageFormat::Pnm => {}
            ImageFormat::Tiff => {}
            ImageFormat::Tga => {}
            ImageFormat::Dds => {}
            ImageFormat::Bmp => {}
            ImageFormat::Ico => {}
            ImageFormat::Hdr => {}
            ImageFormat::OpenExr => {}
            ImageFormat::Farbfeld => {}
            ImageFormat::Avif => {}
            ImageFormat::Qoi => {}
            */

             _ => {
                 let format_hint = ImageFormatHint::from(p);
                 Err(ImageError::Encoding(EncodingError::from_format_hint(format_hint)))
             }
        }
    }

    pub fn save_fmt(&self, path : String, fmt : ImageFmt ) -> EncoderThread
    {

        let p : String = String::from(path);
        // p.push_str(&fmt.get_image_ext().unwrap());

        let image_format = fmt.get_image_format();

        /*encoding is too expensive in execution time (see GIF encoding).
            spawn thread.
            p.clone() in facts we don't know the lifetime of the thread
        */
        let image_formatter = Self::clone(self);
        let image_name = p.clone();

        EncoderThread{
            thread: spawn(move || {
                Self::encoder_thread(image_formatter, p.clone(), image_format)
            }),
            image_name,
        }

    }

    pub fn to_clipboard(&self) -> Result<(), arboard::Error>
    {
        let mut cb = Clipboard::new()?;
        match notifica::notify("Image available in clipboard.", "") {
            Ok(_) => {}
            Err(_) => {}
        }
        cb.set_image(ImageData {
            width: self.width as usize,
            height: self.height as usize,
            bytes: (&self.buffer).into(),
        })

    }
}
