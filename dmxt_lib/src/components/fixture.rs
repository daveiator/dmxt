
use crate::builders::fixture::FixtureModel;
use crate::dmx::{DMXAddress, DMXDevice};
use open_dmx::error::DMXError;
use crate::{address};

pub struct Fixture {
    pub name: String,
    pub address: DMXAddress,
    model: FixtureModel,
}

impl DMXDevice for Fixture {
    fn write_channels(&mut self, channels: &mut [u8]) -> Result<(), DMXError> {
        Ok(())
    }
}

impl Fixture {
    pub fn new(name: String, address: DMXAddress, model: FixtureModel) -> Result<Fixture, DMXError> {
        Ok(Fixture {
            name,
            address,
            model,
        })
    }
}