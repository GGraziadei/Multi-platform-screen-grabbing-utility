use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use mpsg::screenshots::{CaptureArea, ScreenshotExecutor};
use mpsg::configuration::Configuration;
use screenshots::DisplayInfo;
use serde_json::error;

#[test]
fn screenshot_test(){
    let configuration = Configuration::default();
    let (se, thread) = ScreenshotExecutor::new(Arc::new(RwLock::new(configuration)));

    for di in DisplayInfo::all().unwrap().into_iter(){
        let ca = None;
        se.screenshot(di, ca).unwrap();
    }

    assert!(true);
}

#[test]
fn screenshot_test_delay(){
    let mut configuration = Configuration::default();
    let dd = Duration::from_secs(1);
    configuration.set_delay(Some(dd.clone()));

    let (se, thread) = ScreenshotExecutor::new(Arc::new(RwLock::new(configuration)));

    for di in DisplayInfo::all().unwrap().into_iter(){
        let ca = None;
        let i1 = Instant::now();
        se.screenshot(di, ca).unwrap();
        let i2 = Instant::now();
        let d = i2 -i1;
        assert!(d >= dd.clone());
    }

    assert!(true);
}

#[test]
fn screenshot_test_drop_thread(){
    let configuration = Configuration::default();
    let (se, thread) = ScreenshotExecutor::new(Arc::new(RwLock::new(configuration)));

    drop(se);
    thread.thread.join().unwrap();
    assert!(true);
}

#[test]
fn screenshot_test_capture_area_error(){
    let configuration = Configuration::default();
    let (se, thread) = ScreenshotExecutor::new(Arc::new(RwLock::new(configuration)));

    let mut e = 0;
    for di in DisplayInfo::all().unwrap().into_iter(){
        let ca = Some(CaptureArea{
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        });
        match se.screenshot(di, ca){
            Ok(_) => {}
            Err(err) => {
                e += 1;
            }
        }
    }

    assert_eq!(e, DisplayInfo::all().unwrap().len());
}

#[test]
fn screenshot_test_capture_area(){
    let configuration = Configuration::default();
    let (se, thread) = ScreenshotExecutor::new(Arc::new(RwLock::new(configuration)));

    let mut e = 0;
    for di in DisplayInfo::all().unwrap().into_iter(){
        let ca = Some(CaptureArea{
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        });
        match se.screenshot(di, ca){
            Ok(_) => {
                e += 1;

            } Err(err) => { }
        }
    }

    assert_eq!(e, DisplayInfo::all().unwrap().len());
}