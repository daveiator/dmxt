use serial;
use serial::SerialPort;

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
            loop {
                agent.channels = rx.try_recv().unwrap_or(agent.channels);
                agent.send_dmx_packet().unwrap();
                // println!("{:?}", agent.channels); //Debug
            }
        });
        Ok(dmx)
    }

    pub fn set_channel(&mut self, channel: usize, value: u8) -> Result<(), String> {
        check_valid_channel(channel)?;
        if self.channels.len() < channel {
            self.channels.resize(channel, 0);
        }
        self.channels[channel - 1] = value;
        self.tx.send(self.channels.clone()).unwrap();
        Ok(())
    }

    pub fn get_channel(&self, channel: usize) -> Result<u8, String> {
        check_valid_channel(channel)?;
        if channel > self.channels.len() {
            return Err(format!("Channel {} is not initialized", channel));
        }
        Ok(self.channels[channel - 1])
    }

    pub fn get_channels(&self) -> &Vec<u8> {
        &self.channels
    }

    pub fn set_max_channels(&mut self, max_channels: usize) -> Result<(), String> {
        check_valid_channel(max_channels)?;
        self.channels.resize(max_channels, 0);
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
        let channels = &self.channels;
        //self.send_data(&channels)?;
        Ok(())
    }
}



fn check_valid_channel(channel: usize) -> Result<(), String> {
    if channel > 512 {
        return Err(format!("Channel {} is out of range", channel));
    }
    Ok(())
}