use std::sync::{Arc, RwLock};
use mpsg::configuration::Configuration;
use mpsg::image_combiner::ImageCombiner;
use mpsg::screenshots::ScreenshotExecutor;

#[test]
fn image_combiner_test(){
    let configuration = Configuration::default();
    let (se, thread) = ScreenshotExecutor::new(Arc::new(RwLock::new(configuration)));

    let s = se.screenshot_all();
    assert!(s.is_some());
    let r = ImageCombiner::combine(s.unwrap());

    assert!(r.is_some());
}