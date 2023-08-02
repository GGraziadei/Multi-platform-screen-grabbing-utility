use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::sync::mpsc::{Receiver, RecvError, SendError, sync_channel, SyncSender};
use std::thread;
use std::thread::{Builder, JoinHandle, spawn};
use std::time::Duration;
use anyhow::Error;
use log::info;
use screenshots::{DisplayInfo, Image, Screen};
use thread_priority::{set_current_thread_priority, ThreadPriority};
use crate::configuration::Configuration;

pub struct CaptureArea{
    x : i32,
    y: i32,
    width: u32,
    height: u32
}

impl CaptureArea{
    pub fn new(x : i32 ,y : i32,width : u32,height:u32) -> Option<Self>
    {
        Some(Self{
            x,
            y,
            width,
            height,
        })
    }
}

struct PrintData{
    di : Option<DisplayInfo>,
    ca : Option<CaptureArea>,
    all_screen: bool
}

enum ScreenshotMessage {
    Print(PrintData),
    Image(anyhow::Result<Image>),
    Images(Vec<anyhow::Result<Image>>)
}

const REQUESTS: usize = 5;

pub struct ScreenshotExecutorThread{
    pub thread: JoinHandle<usize>
}

pub struct ScreenshotExecutor{
    rx : Receiver<ScreenshotMessage>,
    tx : SyncSender<ScreenshotMessage>
}

impl ScreenshotExecutor{

    fn thread_executor_delay(pd : Option<Duration>) -> ()
    {
        match pd {
            None => {}
            Some(duration) => {
                info!("ScreenshotExecutor : delay {:?}", duration);
                thread::sleep(duration);
            }
        }
    }

    fn thread_executor(tx : SyncSender<ScreenshotMessage>, rx : Receiver<ScreenshotMessage>, configuration: Arc<RwLock<Configuration>>) -> usize
    {
        //assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        info!("ScreenshotExecutor: thread_executor start");
        loop {
            match rx.recv(){
                Ok(msg) => {
                    if let ScreenshotMessage::Print( pd) = msg {

                        let configuration_lock = configuration.read().unwrap();
                        Self::thread_executor_delay(configuration_lock.get_delay());
                        drop(configuration_lock);

                        /*Results message*/
                        let mut msg : ScreenshotMessage;

                        if pd.all_screen {
                            let screens = Screen::all().unwrap();
                            let mut results = Vec::<anyhow::Result<Image>>::with_capacity(screens.len());
                            for s in screens.into_iter()  {
                                results.push(s.capture());
                            }
                            msg = ScreenshotMessage::Images(results);
                        }
                        else{
                            let s = Screen::new(&pd.di.unwrap());
                            let img = match pd.ca {
                                None => {s.capture()}
                                Some(area) => {
                                    s.capture_area(area.x,area.y,area.width,area.height)
                                }
                            };
                            msg = ScreenshotMessage::Image(img);
                        }

                        match tx.send(msg)  {
                            Ok(()) => {}
                            Err(e) => {
                                info!("ScreenshotExecutor: thread_executor return.");
                                return 1;
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("ScreenshotExecutor: thread_executor return.");
                    return 0;
                }
            }
        }
    }


    pub fn new(configuration: Arc<RwLock<Configuration>>) -> (Self, ScreenshotExecutorThread)
    {
        /*channel from ti to the thread screenshot executor*/
        let (tx, rx) = sync_channel::<ScreenshotMessage>(REQUESTS);

        /*channel from the thread screenshot executor to ti*/
        let (tx_t, rx_t) = sync_channel::<ScreenshotMessage>(REQUESTS);

        /*thread executor*/
        let thread : JoinHandle<usize> = spawn(move || Self::thread_executor(tx_t, rx, configuration));

        (Self{
            rx: rx_t,
            tx
        }, ScreenshotExecutorThread{
            thread,
        })
    }

    pub fn screenshot(&self, di : DisplayInfo, area : Option<CaptureArea> ) -> anyhow::Result<Image>
    {

        /*Each thread can have own sender. MSSR */
        let tx = self.tx.clone();

        let pd = PrintData{
            di : Some(di),
            ca: area,
            all_screen: false,
        };

        let m_send = ScreenshotMessage::Print(pd);
        tx.send(m_send)?;

        match  self.rx.recv()?{
            ScreenshotMessage::Image(img) => {
                drop(tx);
                img
            }
            _ => {
                panic!("Error in intra-thread message format");
            }
        }
    }

    pub fn screenshot_all(&self) -> Option<Vec<anyhow::Result<Image>>>
    {
        /*Each thread can have own sender. MSSR */
        let tx = self.tx.clone();

        let pd = PrintData{
            ca: None,
            all_screen: true,
            di: None,
        };

        let m_send = ScreenshotMessage::Print(pd);
        tx.send(m_send).ok()?;

        if let ScreenshotMessage::Images(img) = self.rx.recv().ok()?{
            /*Explicit drop of tx*/
            drop(tx);
            return Some(img);
        }

        /*Explicit drop of tx*/
        drop(tx);
        None
    }

}

impl Drop for ScreenshotExecutor{
    fn drop(&mut self) {
        /*when drop ScreenshotExecutor also the tx is dropped. This produce the return of the executor thread*/
        info!("Drop: ScreenshotExecutor");
    }
}
