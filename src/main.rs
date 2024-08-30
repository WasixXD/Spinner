use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::borrow::Cow;
use std::io::{Write, self};
use crossterm::{
    execute,
    cursor::{MoveToRow, SavePosition},
};

#[derive(Debug)]
struct Spinner {
    pallete: Arc<Box<[Cow<'static, str>]>>,
    label: Arc<String>,
    running: Arc<AtomicBool>,
    pause: Arc<Duration>,
    position: Arc<usize>,
}


impl Spinner {

    pub fn new(label: String, position: usize) -> Arc<Self> {
        Arc::new(Self {
            pallete: Arc::new(Box::new([
                Cow::Borrowed("|"),
                Cow::Borrowed("/"),
                Cow::Borrowed("-"),
                Cow::Borrowed("\\"),
            ])),
            label: Arc::new(label),
            running: Arc::new(AtomicBool::new(false)),
            pause: Arc::new(Duration::from_millis(150)),
            position: Arc::new(position)
        })
    }

    pub fn start(self: Arc<Self>) {
        self.running.store(true, Ordering::Relaxed);
        let spinner_clone = Arc::clone(&self);
        thread::spawn(move || {
            while spinner_clone.running.load(Ordering::Relaxed) {
                for p in spinner_clone.pallete.iter() {

                    let _ = execute!(
                        io::stdout(),
                        SavePosition,
                        MoveToRow((*spinner_clone.position).try_into().unwrap()),
                    );

                    print!("\r{p} {}", spinner_clone.label);
                    let _ = io::stdout().flush();
                    thread::sleep(*spinner_clone.pause);
                }
            }
            println!("\n");
        });

    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

fn main() {
    let s1 = Spinner::new("Downloading...".into(), 1);
    let s2 = Spinner::new("Executing...".into(), 2);
    let s3 = Spinner::new("Closing...".into(), 3);
    let s4 = Spinner::new("Fetching...".into(), 4);

    let spinner = s1.clone();
    let spinner2 = s2.clone();
    let spinner3 = s3.clone();
    let spinner4 = s4.clone();

    let mut handles: Vec<_> = Vec::new();

    s1.start();
    s2.start();
    s3.start();
    s4.start();
    
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        spinner.stop();
    }));
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_secs(7));
        spinner2.stop();
    }));
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        spinner3.stop();
    }));
    handles.push(thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        spinner4.stop();
    }));
    for e in handles {
        let _ = e.join();
    }
}

