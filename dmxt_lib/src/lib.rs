pub mod dmx;
pub mod threads;
pub mod error;
pub mod timing;
pub mod builders;
pub mod macros;


pub fn check_valid_channel(channel: usize) -> Result<(), error::DMXError> {
    if channel > dmx::DMX_CHANNELS {
        return Err(error::DMXError::NotValid(error::DMXErrorValidity::TooHigh));
    }
    if channel < 1 {
        return Err(error::DMXError::NotValid(error::DMXErrorValidity::TooLow));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}