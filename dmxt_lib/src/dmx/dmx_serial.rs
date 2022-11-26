use crate::threads::shared::Lock;
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

// Minimum time between break and break
const MIN_TIME_BREAK_TO_BREAK: time::Duration = time::Duration::from_micros(40_000);


/// A Serial DMX-Interface which writes to the Serial-Port independently from the main thread. 
#[derive(Debug)]
pub struct DMXSerial {
    // Array of DMX-Values which are written to the Serial-Port
    channels: Lock<[u8; DMX_CHANNELS]>,
    // Connection to the Agent-Thread, if this is dropped the Agent-Thread will stop
    _tx: mpsc::Sender<()>,

}

impl DMXSerial {
    /// Opens a new DMX-Interface on the given Serial-Port path. Returns an error if the port could not be opened.
    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<DMXSerial, serial::Error> {
        let (_tx, rx) = mpsc::channel();
        let dmx = DMXSerial { channels: Lock::new([0; DMX_CHANNELS]), _tx}; // channel default created here!
        let mut agent = DMXSerialAgent::init(port)?;
        let channel_view = dmx.channels.read_only();
        let _ = thread::spawn(move || {
                thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max).unwrap_or_else(|e| {
                    eprintln!("Failed to set thread priority: \"{:?}\". Continuing anyways...", e)
                });
                loop {
                    match rx.try_recv() {
                        Err(mpsc::TryRecvError::Disconnected) => break,
                        _ => {
                            let channels = channel_view.read().unwrap().clone();
                            // println!("{:?}", channels); //Debug
                            agent.send_dmx_packet(channels).unwrap();
                        }
                    }
                }
        });
        Ok(dmx)
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) -> Result<(), DMXError> {
        check_valid_channel(channel)?;
        let mut channels = self.channels.write().unwrap();
        channels[channel - 1] = value;
        Ok(())
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
}

struct DMXSerialAgent {
    port: serial::SystemPort,
}

impl DMXSerialAgent {

    pub fn init<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<DMXSerialAgent, serial::Error> {
        let port = serial::SystemPort::open(port)?;
        let dmx = DMXSerialAgent {
            port: port,
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

    #[cfg(feature = "short_dmx")]
    pub fn send_dmx_packet(&mut self, channels: [u8; DMX_CHANNELS]) -> serial::Result<()> {
        let start = time::Instant::now();
        self.send_break()?;
        thread::sleep(TIME_BREAK_TO_DATA);
        let last_not_zero = channels.iter().rposition(|&x| x != 0).unwrap_or(0);
        let mut prefixed_data = [0; 513];// 1 start byte + 512 channels
        prefixed_data[1..].copy_from_slice(&channels);
        self.send_data(&prefixed_data[..last_not_zero])?;
        #[cfg(profile = "dev")]
        print!("\rTime: {:?} ", start.elapsed());
        thread::sleep(MIN_TIME_BREAK_TO_BREAK.saturating_sub(start.elapsed()));
        #[cfg(profile = "dev")]
        print!("Time to send: {:?}", start.elapsed());
        Ok(())
    }

    #[cfg(not(feature = "short_dmx"))]
    pub fn send_dmx_packet(&mut self, channels: [u8; DMX_CHANNELS]) -> serial::Result<()> {
        let start = time::Instant::now();
        self.send_break()?;
        thread::sleep(TIME_BREAK_TO_DATA);
        let mut prefixed_data = [0; 513];// 1 start byte + 512 channels
        prefixed_data[1..].copy_from_slice(&channels);
        self.send_data(&prefixed_data)?;

        #[cfg(profile = "dev")]
        print!("\rTime: {:?} ", start.elapsed());

        thread::sleep(MIN_TIME_BREAK_TO_BREAK.saturating_sub(start.elapsed()));

        #[cfg(profile = "dev")]
        print!("Time to send: {:?}", start.elapsed());

        Ok(())
    }
}