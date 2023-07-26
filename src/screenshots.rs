use std::sync::{Condvar, Mutex};
use std::sync::mpsc::{Receiver, RecvError, SendError, sync_channel, SyncSender};
use std::thread;
use std::thread::{JoinHandle, spawn};
use std::time::Duration;
use screenshots::{DisplayInfo, Image, Screen};

struct PrintData{
    delay : Option<Duration>,
    di : DisplayInfo
}

enum ScreenshotMessage {
    Print(PrintData),
    Image(Image)
}

pub struct ScreenshotExecutor{
    rx : Receiver<ScreenshotMessage>,
    tx : SyncSender<ScreenshotMessage>,
    thread : JoinHandle<usize>
}

const REQUESTS: usize = 5;

impl ScreenshotExecutor{

    fn thread_executor_delay(pd : Option<Duration>) -> ()
    {
        match pd {
            None => {}
            Some(duration) => {
                println!("ScreenshotExecutor : delay {:?}", duration);
                thread::sleep(duration);
            }
        }
    }

    fn thread_executor(tx : SyncSender<ScreenshotMessage>, rx : Receiver<ScreenshotMessage>) -> usize
    {
        println!("ScreenshotExecutor: thread_executor start");
        loop {
            match rx.recv(){
                Ok(msg) => {
                    if let ScreenshotMessage::Print( pd) = msg {
                        Self::thread_executor_delay(pd.delay);
                        let s = Screen::new(&pd.di);
                        let img = s.capture().unwrap();
                        let msg = ScreenshotMessage::Image(img);
                        match tx.send(msg)  {
                            Ok(()) => {}
                            Err(e) => {
                                println!("ScreenshotExecutor: thread_executor return.");
                                return 1;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("ScreenshotExecutor: thread_executor return.");
                    return 0;
                }
            }
        }
    }


    pub fn new() -> Self
    {
        /*channel from ti to the thread screenshot executor*/
        let (tx, rx) = sync_channel::<ScreenshotMessage>(REQUESTS);

        /*channel from the thread screenshot executor to ti*/
        let (tx_t, rx_t) = sync_channel::<ScreenshotMessage>(REQUESTS);

        /*thread executor*/
        let thread : JoinHandle<usize> = spawn(move || Self::thread_executor(tx_t, rx));

        Self{
            rx: rx_t,
            tx,
            thread
        }
    }

    pub fn screenshot(&self, di : DisplayInfo, delay : Option<Duration> ) -> Option<Image>
    {

        /*Each thread can have own sender. MSSR */
        let tx = self.tx.clone();

        let pd = PrintData{
            delay,
            di,
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
        println!("Drop: ScreenshotExecutor");
    }
}