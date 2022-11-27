use crate::threads::shared::Lock;

use std::collections::VecDeque;
use std::time;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
pub type BPM = f64;

pub struct Metronome {
    bpm: Lock<BPM>,
    content: MetronomeContent,

    buffer_cap: u8,
    bpm_buffer: VecDeque<BPM>,
    last_tap: Option<time::Instant>,
}

impl Metronome {
    pub fn new(bpm: BPM) -> Metronome {
        Metronome {
            bpm: Lock::new(bpm),
            content: MetronomeContent::Callback(Arc::new(Mutex::new(|| {}))),
            buffer_cap: 16,
            bpm_buffer: VecDeque::with_capacity(u8::MAX as usize + 1),
            last_tap: None,
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

    pub fn tap(&mut self) {
        let now = time::Instant::now();
        if let Some(last) = self.last_tap.replace(now) {
            let time_elapsed = now.saturating_duration_since(last).as_secs_f64();
            self.bpm_buffer.push_back(60.0 / time_elapsed);

            while self.bpm_buffer.len() > self.buffer_cap as usize {
                self.bpm_buffer.pop_front();
            }
            
            self.update_bpm();
        }
            
    }

    fn update_bpm(&mut self) {
        let mut sum = 0.0;
        for bpm in &self.bpm_buffer {
            sum += bpm;
        }
        let avg = sum / self.bpm_buffer.len() as f64;
        self.set_bpm(avg).unwrap();
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