use egui::ColorImage;
use image::ImageResult;
use mpsg::configuration::ImageFmt;
use mpsg::image_formatter::ImageFormatter;

#[test]
fn image_formatter_from_image(){
    let image = screenshots::Image::new(0,0,vec![]);
    let imf = ImageFormatter::from(image);
}

#[test]
fn image_formatter_from_color_image(){
    let image = ColorImage::example();
    let imf = ImageFormatter::from(image);
}

#[test]
fn image_formatter_render_png(){
    let image = ColorImage::example();
    let imf = ImageFormatter::from(image);
    let thread = imf.save_fmt("target/test".to_string(), ImageFmt::PNG);

    assert_eq!("target/test".to_string(), thread.get_image_name().clone() );
    let mut res = false;
    match thread.thread.join().expect("join error"){
        Ok(()) => {
            res = true;
        }
        Err(_) => {}
    }
    assert!(res);
}

#[test]
fn image_formatter_render_gif(){
    let image = ColorImage::example();
    let imf = ImageFormatter::from(image);
    let thread = imf.save_fmt("target/test".to_string(), ImageFmt::GIF);

    assert_eq!("target/test".to_string(), thread.get_image_name().clone() );
    let mut res = false;
    match thread.thread.join().expect("join error"){
        Ok(()) => {
            res = true;
        }
        Err(_) => {}
    }
    assert!(res);
}

#[test]
fn image_formatter_render_jpg(){
    let image = ColorImage::example();
    let imf = ImageFormatter::from(image);
    let thread = imf.save_fmt("target/test".to_string(), ImageFmt::JPG);

    assert_eq!("target/test".to_string(), thread.get_image_name().clone() );
    let mut res = false;
    match thread.thread.join().expect("join error"){
        Ok(()) => {
            res = true;
        }
        Err(_) => {}
    }
    assert!(res);
}

#[test]
fn image_formatter_to_clipboard(){
    let image = ColorImage::example();
    let imf = ImageFormatter::from(image);
    imf.to_clipboard().expect("error clipboard");

    assert!(true);
}


