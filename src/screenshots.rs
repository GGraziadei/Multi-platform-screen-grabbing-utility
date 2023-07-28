use std::sync::{Condvar, Mutex};
use std::sync::mpsc::{Receiver, RecvError, SendError, sync_channel, SyncSender};
use std::thread;
use std::thread::{Builder, JoinHandle, spawn};
use std::time::Duration;
use log::info;
use screenshots::{DisplayInfo, Image, Screen};
use thread_priority::{set_current_thread_priority, ThreadPriority};

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
    delay : Option<Duration>,
    di : DisplayInfo,
    ca : Option<CaptureArea>
}

enum ScreenshotMessage {
    Print(PrintData),
    Image(Image)
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

    fn thread_executor(tx : SyncSender<ScreenshotMessage>, rx : Receiver<ScreenshotMessage>) -> usize
    {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        info!("ScreenshotExecutor: thread_executor start");
        loop {
            match rx.recv(){
                Ok(msg) => {
                    if let ScreenshotMessage::Print( pd) = msg {
                        Self::thread_executor_delay(pd.delay);
                        let s = Screen::new(&pd.di);
                        let img = match pd.ca {
                            None => {s.capture().unwrap()}
                            Some(area) => {
                                s.capture_area(area.x,area.y,area.width,area.height).unwrap()
                            }
                        } ;
                        let msg = ScreenshotMessage::Image(img);
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


    pub fn new() -> (Self, ScreenshotExecutorThread)
    {
        /*channel from ti to the thread screenshot executor*/
        let (tx, rx) = sync_channel::<ScreenshotMessage>(REQUESTS);

        /*channel from the thread screenshot executor to ti*/
        let (tx_t, rx_t) = sync_channel::<ScreenshotMessage>(REQUESTS);

        /*thread executor*/
        let thread : JoinHandle<usize> = spawn(move || Self::thread_executor(tx_t, rx));

        (Self{
            rx: rx_t,
            tx
        }, ScreenshotExecutorThread{
            thread,
        })
    }

    pub fn screenshot(&self, di : DisplayInfo, delay : Option<Duration>, area : Option<CaptureArea> ) -> Option<Image>
    {

        /*Each thread can have own sender. MSSR */
        let tx = self.tx.clone();

        let pd = PrintData{
            delay,
            di,
            ca: area,
        };

        let m_send = ScreenshotMessage::Print(pd);
        tx.send(m_send).ok()?;

        if let ScreenshotMessage::Image( img) = self.rx.recv().ok()?{
            /*Explicit drop of tx*/
            drop(tx);
            return Some(img);
        }

        /*Explicit drop of tx*/
        drop(tx);
        return None;
    }

}

impl Drop for ScreenshotExecutor{
    fn drop(&mut self) {
        /*when drop ScreenshotExecutor also the tx is dropped. This produce the return of the executor thread*/
        info!("Drop: ScreenshotExecutor");
    }
}