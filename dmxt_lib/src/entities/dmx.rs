//Lowest level

use crate::dmx_serial::DMXSerial;

pub struct Universe {
    id: u32,
    name: String,
    description: String,
    serial: DMXSerial,
}

impl Universe {
    pub fn new(id: u32, name: String, description: String, serial: DMXSerial) -> Universe {
        Universe {
            id,
            name,
            description,
            serial,
        }
    }
}