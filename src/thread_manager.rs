use std::collections::HashSet;
use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{SendError, sync_channel};
use std::thread;
use egui::Key::K;
use log::{error, info};
use crate::configuration::Configuration;
use crate::gui::GuiThread;
use crate::image_formatter::EncoderThread;
use crate::screenshots::{ScreenshotExecutor, ScreenshotExecutorThread};

pub struct ThreadManager{
    encoders: Arc<Mutex<Vec<EncoderThread>>>,
    screenshots_executor : ScreenshotExecutorThread,
}

impl ThreadManager{

    fn init() -> (Arc<RwLock<Configuration>>,Arc<Mutex<Vec<EncoderThread>>>)
    {
        // assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        env_logger::init();
        (Arc::new(RwLock::new(Configuration::new()))
            ,Arc::new(Mutex::new(Vec::<EncoderThread>::new())))
    }

    pub fn new() -> ()
    {

        let (configuration, encoders) = Self::init();
        let (screenshot_executor,executor_thread) = ScreenshotExecutor::new(configuration.clone());

        let tm = ThreadManager{
            encoders: encoders.clone(),
            screenshots_executor: executor_thread,
        };
        

        let mut keys = HashSet::new();
        let (tx, rx) = sync_channel::<HashSet<Key>>(1);

        thread::spawn(move || {
            listen(move |e| {
                println!("{:?}", keys);
                match e.event_type {
                    EventType::KeyPress(k) => {
                        keys.insert(k);
                        match tx.send(keys.clone()){
                            Ok(_) => {}
                            Err(e) => { println!("{}", e); }
                        }
                    }
                    EventType::KeyRelease(k) => {
                        keys.remove(&k);
                        match tx.send(keys.clone()){
                            Ok(_) => {}
                            Err(e) => { println!("{}", e); }
                        }
                    }
                    _ => {
                        keys.clear()
                    }
                }
            }).expect("TODO: panic message");
        });
        /*GuiThread is mapped over the main thread (ThreadManager)*/
        GuiThread::new(configuration.clone(), encoders, screenshot_executor, rx);

        /*When GuiThread return event loop is closed. ScreenshotExecutor is dropped.
        thread_executor returns.*/
        tm.screenshots_executor.thread.join()
            .expect("Error during screenshots_executor join");

        /*Wait all encoder_threads end their work. Without this join if an encoder haven't finished yet
        its work when main thread will go down also it will go down and the work is not completed.*/
        let encoders =  Arc::try_unwrap(tm.encoders)
            .expect("Error in encoders access").into_inner()
            .expect("Error in encoders lock");

        for e in encoders
        {
            info!("check status encoder_thread for {}", e.get_image_name());
            match e.thread.join().expect("Error during encoder join") {
                Ok(()) => {
                    info!("thread joined.");
                }
                Err(e) => {
                    error!("Error error {}",e);
                }
            }
        }
        info!("slave threads down");
    }
}
