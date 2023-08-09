use std::sync::{Arc, Mutex, RwLock};
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

    pub fn new() -> Self
    {

        let (configuration, encoders) = Self::init();
        let (screenshot_executor,executor_thread) = ScreenshotExecutor::new(configuration.clone());

        let tm = ThreadManager{
            encoders: encoders.clone(),
            screenshots_executor: executor_thread,
        };

        /*GuiThread is mapped over the main thread (ThreadManager)*/
        GuiThread::new(configuration.clone(), encoders.clone(), screenshot_executor);

        tm
    }

    pub fn join(self) -> ()
    {
        let encoders = Arc::try_unwrap(self.encoders)
            .expect("Error in encoders access").into_inner()
            .expect("Error in encoders lock");

        let screenshots_executor = self.screenshots_executor;

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

        screenshots_executor.thread.join()
            .expect("Error during screenshots_executor join");

        info!("slave threads down");
    }

}
