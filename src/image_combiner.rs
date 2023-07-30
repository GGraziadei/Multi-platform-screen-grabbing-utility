use image::{DynamicImage, ImageBuffer, imageops};
use log::info;
use screenshots::Image;

pub struct ImageCombiner;

impl ImageCombiner {

    pub fn combine(value: Vec<Image>) -> Option<Image>
    {
        let mut width : u32 = 0;
        let mut height : u32 = 0;

        for i in value.iter()
        {
            width += i.width();
            if height < i.height(){
                height = i.height()
            }
        }

        let mut buffer_image = DynamicImage::new_rgba8(width, height);
        let mut offset = 0;
        info!("Image combining start.");

        for (index,i) in value.into_iter().rev().enumerate()
        {
            info!("Combine image {} entry point {}", index,offset);
            let img = ImageBuffer::from_raw(i.width(), i.height(),i.rgba().as_slice())?;
            imageops::overlay(&mut buffer_image,&img,offset,0);
            offset += ( i.width() as i64);
        }

        info!("Image combining end.");
        Some(Image::new(
            width,
            height,
            buffer_image.into_bytes(),
        ))
    }


}