use lazy_static::lazy_static;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Condvar, Mutex};
use std::thread;

use bincode;

use crate::events::{Event, EventKind};

lazy_static! {
    pub static ref TX: SyncSender<Event> = {
        let (tx, rx) = mpsc::sync_channel(1024);
        thread::spawn(|| backend_thread(rx));
        tx
    };
    static ref FINISHED: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());
}

pub fn init() {}

pub fn finalize() {
    // Notify the backend that we're done
    TX.send(Event::done()).unwrap();

    // Wait for the backend thread to finish
    let (ref lock, ref cvar) = &*FINISHED;
    let mut finished = lock.lock().unwrap();
    while !*finished {
        finished = cvar.wait(finished).unwrap();
    }
}

fn backend_thread(rx: Receiver<Event>) {
    let (ref lock, ref cvar) = &*FINISHED;
    let mut finished = lock.lock().unwrap();

    (match env::var("INSTRUMENT_BACKEND").unwrap_or_default().as_str() {
        "log" => log,
        "debug" => debug,
        _ => debug,
    })(rx);

    *finished = true;
    cvar.notify_one();
}

fn log(rx: Receiver<Event>) {
    let path = env::var("INSTRUMENT_OUTPUT")
        .expect("Instrumentation requires the INSTRUMENT_OUTPUT environment variable be set");
    let mut out = BufWriter::new(
        File::create(&path).unwrap_or_else(|_| panic!("Could not open output file: {:?}", path)),
    );

    for event in rx {
        if let EventKind::Done = event.kind {
            return;
        }
        bincode::serialize_into(&mut out, &event).unwrap();
    }
}

fn debug(rx: Receiver<Event>) {
    for event in rx {
        eprintln!("{:?}", event);
        if let EventKind::Done = event.kind {
            return;
        }
    }
}
