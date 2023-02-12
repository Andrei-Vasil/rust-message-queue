use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex}, thread};
use super::worker::Worker;


pub struct Daemon {
    tx: Arc<Mutex<Sender<bool>>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Daemon {
    pub fn new(workers: Arc<Mutex<Vec<(Worker, Receiver<bool>)>>>) -> Self {
        let (tx, rx) = mpsc::channel();
        let handle = Daemon::init_daemon_thread(rx, workers);
        Self { tx: Arc::new(Mutex::new(tx)), handle }
    }

    fn init_daemon_thread(rx: Receiver<bool>, workers: Arc<Mutex<Vec<(Worker, Receiver<bool>)>>>) -> Option<thread::JoinHandle<()>> {
        Some(thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(_) => {
                    break;
                },
                Err(_) => {},
            };

            let mut guard = workers.lock().unwrap();
            let _workers = &mut *guard;
            _workers.retain(|(worker, rx)| {
                match rx.try_recv() {
                    Ok(_) => {
                        drop(worker);
                        false
                    },
                    Err(_) =>true,
                }
            });
        }))
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        self.tx.lock().unwrap().send(true);
        if let Some(thread) = self.handle.take() {
            thread.join().unwrap();
        }
    }
}