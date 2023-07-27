use log::error;
use crate::image_formatter::EncoderThread;
use crate::screenshots::{ScreenshotExecutor, ScreenshotExecutorThread};

pub struct ThreadManager{
    encoders: Vec<EncoderThread>,
    screenshots_executor : ScreenshotExecutorThread
}


impl ThreadManager{

    pub fn new() -> (ScreenshotExecutor, Self)
    {

        let (s, executor_thread) = ScreenshotExecutor::new();
        let tm = ThreadManager{
            encoders: vec![],
            screenshots_executor: executor_thread,
        };

        (s,tm)
    }

    pub fn add_encoder(&mut self, e : EncoderThread)
    {
        self.encoders.push(e)
    }

    pub fn join(self) -> ()
    {
        let encoders = self.encoders;
        let screenshots_executor = self.screenshots_executor;

        for e in encoders.into_iter(){
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
    }

}