
use std::u8;

use crate::builders::fixture::FixtureName;

use serde::{Serialize, Deserialize};

use crate::check_valid_channel;
use crate::error::DMXError;

pub const DMX_CHANNELS: usize = 512;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DMXUniverse {
    pub channels: [u8; DMX_CHANNELS],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DMXRange {
    pub start: DMXAddress,
    pub end: DMXAddress,
}

impl DMXRange {
    pub fn new(start: DMXAddress, end: DMXAddress) -> DMXRange{
        DMXRange { start, end }
    }

    pub fn from_tuple(range: (DMXAddress, DMXAddress)) -> DMXRange {
        DMXRange { start: range.0, end: range.1 }
    }
}

impl From<Channel> for DMXRange {
    fn from(channel: Channel) -> Self {
        DMXRange {
            start: (channel, u8::MIN).into(),
            end: (channel, u8::MAX).into(),
        }
    }
}

impl From<(DMXAddress, DMXAddress)> for DMXRange {
    fn from(addresses: (DMXAddress, DMXAddress)) -> Self {
        DMXRange { start: addresses.0, end: addresses.1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DMXAddress {
    pub channel: Channel,
    pub value: u8,
}

impl DMXAddress {
    pub fn new(channel: Channel, value: u8) -> DMXAddress {
        Self {
            channel,
            value,
        }
    }

    pub fn from_tuple(address: (Channel, u8)) -> DMXAddress {
        Self {
            channel: address.0,
            value: address.1,
        }
    }
}

impl From<(Channel, u8)> for DMXAddress {
    fn from((channel, value): (Channel, u8)) -> Self {
        Self {
            channel,
            value,
        }
    }
}

impl From<(u16, u8)> for DMXAddress {
    fn from((channel, value): (u16, u8)) -> Self {
        Self {
            channel: channel.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Channel{
    id: u16
}

impl Channel {
    pub fn new(channel: u16) -> Result<Channel, DMXError> {
        check_valid_channel(channel.into())?;
        Ok(Channel { id: channel })
    }
}

impl From<Channel> for u16 {
    fn from(channel: Channel) -> Self {
        channel.id
    }
}

impl From<u16> for Channel {
    fn from(channel: u16) -> Self {
        Channel { id: channel }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
    Black,
    CustomRGB(String, (u8, u8, u8)),
    Custom(FixtureName),
    UV,
    ColorChange,
    Auto,
    All,
}