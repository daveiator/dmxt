use crate::threads::shared::{Lock, ReadOnly};
use crate::check_valid_channel;
use crate::error::DMXError;
use crate::dmx::DMX_CHANNELS;

use serial;
use serial::SerialPort;

use thread_priority;

use std::time;
use std::io::Write;
use std::ffi::OsStr;
use std::thread;
use std::sync::mpsc;

// Holds the serial port settings for the Break-Signals
const BREAK_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud57600,
    char_size: serial::Bits7,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// Holds the serial port settings for the Data-Signals
const DMX_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::BaudOther(250_000),
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop2,
    flow_control: serial::FlowNone,
};

// Sleep duration between sending the break and the data
const TIME_BREAK_TO_DATA: time::Duration = time::Duration::new(0, 136_000);

/// A Serial DMX-Interface which writes to the Serial-Port independently from the main thread. 
#[derive(Debug)]
pub struct DMXSerial {
    // Array of DMX-Values which are written to the Serial-Port
    channels: Lock<[u8; DMX_CHANNELS]>,
    // Connection to the Agent-Thread, if this is dropped the Agent-Thread will stop
    agent: mpsc::Sender<()>,
    agent_rec: mpsc::Receiver<()>,

    // Mode
    is_sync: Lock<bool>,

    min_time_break_to_break: Lock<time::Duration>,

}

impl DMXSerial {
    /// Opens a new DMX-Interface on the given Serial-Port path. Returns an error if the port could not be opened.
    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<DMXSerial, serial::Error> {

        let (handler, agent_rec) = mpsc::sync_channel(0);
        let (agent, handler_rec) = mpsc::channel();

        // channel default created here!
        let dmx = DMXSerial {
            channels: Lock::new([0; DMX_CHANNELS]),
            agent,
            agent_rec,
            is_sync: Lock::new(false),
            min_time_break_to_break: Lock::new(time::Duration::from_micros(22_700))};

        let mut agent = DMXSerialAgent::init(port, dmx.min_time_break_to_break.read_only())?;
        let channel_view = dmx.channels.read_only();
        let is_sync_view = dmx.is_sync.read_only();
        let _ = thread::spawn(move || {
                thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max).unwrap_or_else(|e| {
                    eprintln!("Failed to set thread priority: \"{:?}\". Continuing anyways...", e)
                });
                loop {
                    if is_sync_view.read().unwrap().clone() {
                        handler_rec.recv().unwrap();
                    }

                    let channels = channel_view.read().unwrap().clone();

                    agent.send_dmx_packet(channels).unwrap();
                    match handler.try_send(()) {
                        Err(mpsc::TrySendError::Disconnected(_)) => break,
                        _ => {}
                    }
                }
        });
        Ok(dmx)
    }

    pub fn open_sync(port: &str) -> Result<DMXSerial, serial::Error> {
        let mut dmx = DMXSerial::open(port)?;
        dmx.set_sync();
        Ok(dmx)
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) -> Result<(), DMXError> {
        check_valid_channel(channel)?;
        let mut channels = self.channels.write().unwrap();
        channels[channel - 1] = value;
        Ok(())
    }

    pub fn set_channels(&mut self, channels: [u8; DMX_CHANNELS]) {
        *self.channels.write().unwrap() = channels;
    }

    pub fn get_channel(&self, channel: usize) -> Result<u8, DMXError> {
        check_valid_channel(channel)?;
        let channels = self.channels.read().unwrap();
        Ok(channels[channel - 1])
    }

    pub fn get_channels(&self) -> [u8; DMX_CHANNELS] {
        self.channels.read().unwrap().clone()
    }

    pub fn reset_channels(&mut self) {
        self.channels.write().unwrap().fill(0);
    }

    pub fn wait_for_update(&self) {
        self.agent_rec.recv().unwrap();
    }

    pub fn update_async(&self) {
        self.agent.send(()).unwrap();
    }

    pub fn update(&mut self) {
        self.update_async();
        self.wait_for_update();
    }

    pub fn set_sync(&mut self) {
        *self.is_sync.write().unwrap() = true;
    }

    pub fn set_async(&mut self) {
        *self.is_sync.write().unwrap() = false;
    }

    pub fn is_sync(&self) -> bool {
        self.is_sync.read().unwrap().clone()
    }

    pub fn is_async(&self) -> bool {
        !self.is_sync()
    }

    pub fn set_packet_time(&mut self, time: time::Duration) {
        self.min_time_break_to_break.write().unwrap().clone_from(&time);
    }

    pub fn get_packet_time(&self) -> time::Duration {
        self.min_time_break_to_break.read().unwrap().clone()
    }

}

pub struct DMXSerialAgent {
    port: serial::SystemPort,
    min_b2b: ReadOnly<time::Duration>,
}

impl DMXSerialAgent {

    pub fn init<T: AsRef<OsStr> + ?Sized>(port: &T, min_b2b: ReadOnly<time::Duration>) -> Result<DMXSerialAgent, serial::Error> {
        let port = serial::SystemPort::open(port)?;
        let dmx = DMXSerialAgent {
            port,
            min_b2b,
        };
        Ok(dmx)
    }
    fn send_break(&mut self) -> serial::Result<()> {
        self.port.configure(&BREAK_SETTINGS)?;
        self.port.write(&[0x00])?;
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> serial::Result<()> {
        self.port.configure(&DMX_SETTINGS)?;
        self.port.write(data)?;
        Ok(())
    }
    
    pub fn send_dmx_packet(&mut self, channels: [u8; DMX_CHANNELS]) -> serial::Result<()> {
        let start = time::Instant::now();
        self.send_break()?;
        thread::sleep(TIME_BREAK_TO_DATA);
        let mut prefixed_data = [0; 513];// 1 start byte + 512 channels
        prefixed_data[1..].copy_from_slice(&channels);
        self.send_data(&prefixed_data)?;

        #[cfg(not(profile = "release"))]
        print!("\rTime: {:?} ", start.elapsed());

        thread::sleep(self.min_b2b.read().unwrap().saturating_sub(start.elapsed()));

        #[cfg(not(profile = "release"))]
        print!("Time to send: {:?}", start.elapsed());

        Ok(())
    }
}