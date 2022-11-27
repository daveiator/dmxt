use crate::threads::shared::Lock;

use std::time;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
pub type BPM = f64;

pub struct Metronome {
    bpm: Lock<BPM>,
    content: MetronomeContent,
}

impl Metronome {
    pub fn new(bpm: BPM) -> Metronome {
        Metronome {
            bpm: Lock::new(bpm),
            content: MetronomeContent::Callback(Arc::new(Mutex::new(|| {}))),
        }
    }

    pub fn set_callback(&mut self, callback: Callback) -> Result<(), MetronomeError>{
        match &self.content {
            MetronomeContent::Callback(_) => {
                self.content = MetronomeContent::Callback(callback);
            },
            MetronomeContent::Channel(tx) => {
                tx.send(MetronomeCommand::NewCallback(callback))?;
            },
        }
        Ok(())
    }

    pub fn set_bpm(&mut self, bpm: BPM) -> Result<(), MetronomeError> {
        self.bpm.write().unwrap().clone_from(&bpm);
        Ok(())
    }

    pub fn get_bpm(&self) -> BPM {
        self.bpm.read().unwrap().clone()
    }

    pub fn start(&mut self) -> Result<(), MetronomeError>{
        if let MetronomeContent::Channel(_) = &self.content {
            return Err(MetronomeError::AlreadyStarted);
        }
        let (tx, rx) = mpsc::channel();
        let callback = match &self.content {
            MetronomeContent::Callback(callback) => callback.clone(),
            MetronomeContent::Channel(_) => unreachable!(),
        };
        self.content = MetronomeContent::Channel(tx);
        let bpm_view = self.bpm.read_only();
        let _ = thread::spawn(move || {
            let mut true_callback = callback;
            loop {
                match rx.try_recv() {
                    Ok(MetronomeCommand::Stop) => {
                        break;
                    },
                    Ok(MetronomeCommand::NewCallback(callback)) => {
                        true_callback = callback;
                    },
                    Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    },
                    _ => {},
                }
                //execute callback
                true_callback.lock().unwrap()();
                let bpm = bpm_view.read().unwrap().clone();
                thread::sleep(beat_duration(bpm));
                
            }
        });
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), MetronomeError>{
        match &self.content {
            MetronomeContent::Channel(tx) => {
                tx.send(MetronomeCommand::Stop)?;
                Ok(())
            },
            MetronomeContent::Callback(_) => {
                return Err(MetronomeError::AlreadyStopped);
            }
        }
    }
}

type Callback = Arc<Mutex<dyn FnMut() + Send + Sync>>;
enum MetronomeContent {
    Callback(Callback),
    Channel(mpsc::Sender<MetronomeCommand>)
}

pub enum MetronomeCommand {
    NewCallback(Callback),
    Stop,
}

#[derive(Debug)]
pub enum MetronomeError {
    AlreadyStarted,
    AlreadyStopped,
    SendError(mpsc::SendError<MetronomeCommand>),
    OtherError(String),
}

impl From<mpsc::SendError<MetronomeCommand>> for MetronomeError {
    fn from(error: mpsc::SendError<MetronomeCommand>) -> Self {
        MetronomeError::SendError(error)
    }
}

impl From<String> for MetronomeError {
    fn from(error: String) -> Self {
        MetronomeError::OtherError(error)
    }
}

fn beat_duration(bpm: BPM) -> time::Duration {
    time::Duration::from_secs_f64(60.0 / bpm)
}