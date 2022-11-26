//All data types that can be saved as a json file

use serde::{Serialize, Deserialize};
use crate::{check_valid_channel, error::DMXError};

#[derive(Serialize, Deserialize)]
pub struct FixtureModel {
    pub name: String,
    pub manufacturer: String,

}


#[derive(Serialize, Deserialize)]
pub struct DMXRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Serialize, Deserialize)]
pub struct DMXAdress {
    pub channel: Channel,
    pub value: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Channel(u16);

impl Channel {
    pub fn new(channel: u16) -> Result<Channel, DMXError> {
        check_valid_channel(channel.into())?;
        Ok(Channel(channel))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Manufacturer {
    pub name: String,
    pub full_name: String,
    pub description: String,
}