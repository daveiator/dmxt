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
    channels: Vec<u8>,
    tx: mpsc::Sender<Vec<u8>>,
}

impl DMXSerial {
    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<DMXSerial, serial::Error> {
        let (tx, rx) = mpsc::channel();
        let dmx = DMXSerial { channels: vec![0], tx}; // channel default created here!
        let mut agent = DMXSerialAgent::init(port, dmx.get_channels())?;
        let _ = thread::spawn(move || {
                thread_priority::set_current_thread_priority(thread_priority::ThreadPriority::Max).unwrap();
                loop {
                    agent.channels = match rx.try_recv() {
                        Ok(channels) => channels,
                        Err(mpsc::TryRecvError::Disconnected) => {
                            println!("DMXSerialAgent: Channel disconnected!");
                            break;
                        },
                        Err(_) => agent.channels,
                    };
                    agent.send_dmx_packet().unwrap();
                    // println!("{:?}", agent.channels); //Debug
                }
        });
        Ok(dmx)
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) -> Result<(), DMXError> {
        check_valid_channel(channel)?;
        if self.channels.len() < channel {
            self.channels.resize(channel, 0);
        }
        self.channels[channel - 1] = value;
        self.tx.send(self.channels.clone()).unwrap();
        Ok(())
    }

    pub fn get_channel(&self, channel: usize) -> Result<u8, DMXError> {
        check_valid_channel(channel)?;
        if channel > self.channels.len() {
            return Err(DMXError::NotInitialized);
        }
        Ok(self.channels[channel - 1])
    }

    pub fn get_channels(&self) -> &Vec<u8> {
        &self.channels
    }

    pub fn set_max_channels(&mut self, max_channels: usize) -> Result<(), DMXError> {
        check_valid_channel(max_channels)?;
        self.channels.resize(max_channels, 0);
        self.tx.send(self.channels.clone()).unwrap();
        Ok(())
    }
}

struct DMXSerialAgent {
    port: serial::SystemPort,
    pub channels: Vec<u8>,
}

impl DMXSerialAgent {

    pub fn init<T: AsRef<OsStr> + ?Sized>(port: &T, parent_channels: &Vec<u8>) -> Result<DMXSerialAgent, serial::Error> {
        let port = serial::SystemPort::open(port)?;
        let dmx = DMXSerialAgent {
            port: port,
            channels: parent_channels.clone(),
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

    pub fn send_dmx_packet(&mut self) -> serial::Result<()> {
        self.send_break()?;
        thread::sleep(SERIAL_TOTAL_BREAK);
        let mut prefixed_data = self.channels.clone();
        prefixed_data.insert(0, 0x00); // DMX start code
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