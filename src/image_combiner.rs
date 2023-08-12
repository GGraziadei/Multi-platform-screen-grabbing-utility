use image::{DynamicImage, ImageBuffer, imageops};
use log::{error, info};
use screenshots::Image;

pub struct ImageCombiner;

impl ImageCombiner {

    pub fn combine(value: Vec<anyhow::Result<Image>>) -> Option<Image>
    {
        let mut width : u32 = 0;
        let mut height : u32 = 0;

        /*
            Check dimension and results of acquisition
        */
        let mut errors = false;
        for image in value.iter()
        {
            match image{
                Ok(i) => {
                    width += i.width();
                    if height < i.height(){
                        height = i.height()
                    }
                }
                Err(error) => {
                    errors = true;
                    error!("{}", error);
                }
            }
        }

        if errors{  return  None; }

        let mut buffer_image = DynamicImage::new_rgba8(width, height);
        let mut offset = 0;
        info!("Image combining start.");

        for (index,image) in value.into_iter().rev().enumerate()
        {
            info!("Combine image {} entry point {}", index,offset);
            match image{
                Ok(i) => {
                    let img = ImageBuffer::from_raw(i.width(), i.height(),i.rgba().as_slice())?;
                    imageops::overlay(&mut buffer_image,&img,offset,0);
                    offset +=  i.width() as i64;
                }
                Err(error) => {
                    /*notifica::notify("Error in screenshot acquisition.", &error.to_string())
                        .expect("OS API error.");*/
                }
            }
        }

        info!("Image combining end.");
        Some(Image::new(
            width,
            height,
            buffer_image.into_bytes(),
        ))
    }


}