use super::threads::shared::Lock;

use serial;
use serial::SerialPort;

use thread_priority;

use std::time;
use std::io::Write;
use std::ffi::OsStr;
use std::thread;
use std::sync::mpsc;


const BREAK_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud57600,
    char_size: serial::Bits7,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

const DMX_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::BaudOther(250_000),
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop2,
    flow_control: serial::FlowNone,
};

const SERIAL_TOTAL_BREAK: time::Duration = time::Duration::new(0, 136_000);



pub struct DMXSerial {
    channels: Lock<[u8; 512]>, //512 channels
    tx: mpsc::Sender<()>,

}

impl DMXSerial {
    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<DMXSerial, serial::Error> {
        let (tx, rx) = mpsc::channel();
        let dmx = DMXSerial { channels: Lock::new([0; 512]), tx}; // channel default created here!
        let mut agent = DMXSerialAgent::init(port)?;
        let channel_view = dmx.channels.read_only();
        let _ = thread::spawn(move || {
                thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max).unwrap();
                loop {
                    match rx.try_recv() {
                        Err(mpsc::TryRecvError::Disconnected) => break,
                        _ => {
                            let channels = channel_view.read().unwrap().clone();
                            println!("{:?}", channels); //Debug
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

    pub fn get_channels(&self) -> [u8; 512] {
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

    pub fn send_dmx_packet(&mut self, channels: [u8; 512]) -> serial::Result<()> {
        self.send_break()?;
        thread::sleep(SERIAL_TOTAL_BREAK);
        let mut prefixed_data = [0; 513]; // 1 start byte + 512 channels
        prefixed_data[1..].copy_from_slice(&channels);
        self.send_data(&prefixed_data)?;
        Ok(())
    }
}



pub fn check_valid_channel(channel: usize) -> Result<(), DMXError> {
    if channel > 512 || channel < 1 {
        return Err(DMXError::NotValid);
    }
    Ok(())
}

#[derive(Debug)]
pub enum DMXError {
    NotInitialized,
    NotValid,
    Other(String)
}

impl std::fmt::Display for DMXError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DMXError::NotInitialized => write!(f, "DMX channel is not initialized"),
            DMXError::NotValid => write!(f, "Channel is not valid ( < 1 or > 512"),
            DMXError::Other(ref s) => write!(f, "{}", s),
        }
    }
} 


impl From<String> for DMXError {
    fn from(err: String) -> DMXError {
        DMXError::Other(err)
    }
}

impl std::error::Error for DMXError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None //I'm laz
    }
}