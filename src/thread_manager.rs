use std::sync::{Arc, Mutex, RwLock};
use log::{error, info};
use thread_priority::{set_current_thread_priority, ThreadPriority};
use crate::configuration::Configuration;
use crate::gui::GuiThread;
use crate::image_formatter::EncoderThread;
use crate::screenshots::{ScreenshotExecutor, ScreenshotExecutorThread};

pub struct ThreadManager{
    configuration : Arc<RwLock<Configuration>>,
    encoders: Arc<Mutex<Vec<EncoderThread>>>,
    screenshots_executor : ScreenshotExecutorThread,
    gui_thread : GuiThread
}

impl ThreadManager{

    fn init() -> (Arc<RwLock<Configuration>>,Arc<Mutex<Vec<EncoderThread>>>)
    {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        env_logger::init();
        (Arc::new(RwLock::new(Configuration::new()))
            ,Arc::new(Mutex::new(Vec::<EncoderThread>::new())))
    }

    pub fn new() -> Self
    {

        let (screenshot_executor,executor_thread) = ScreenshotExecutor::new();
        let (configuration, encoders) = Self::init();

        let tm = ThreadManager{
            configuration : configuration.clone(),
            encoders: encoders.clone(),
            screenshots_executor: executor_thread,
            /*GuiThread is mapped over the main thread (ThreadManager)*/
            gui_thread: GuiThread::new(configuration.clone(), encoders.clone(), screenshot_executor),
        };

        tm
    }

    pub fn add_encoder(&mut self, e : EncoderThread)
    {
        let mut encoders = self.encoders.lock()
            .expect("Error in encoders access");
        encoders.push(e)
    }

    pub fn join(self) -> ()
    {
        let encoders = Arc::try_unwrap(self.encoders)
            .expect("Error in encoders access").into_inner()
            .expect("Error in encoders lock");

        let screenshots_executor = self.screenshots_executor;
        let gui_thread = self.gui_thread;

        for e in encoders
        {
            let encoder_result = e.thread.join()
                .expect("Error during encoder join");
            match encoder_result {
                Ok(_) => {}
                Err(e) => {
                    error!("Error error {}",e);
                }
            }
        }

        screenshots_executor.thread.join()
            .expect("Error during screenshots_executor join");

        info!("slave threads down");
    }

}