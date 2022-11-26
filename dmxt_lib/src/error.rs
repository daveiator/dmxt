#[derive(Debug)]
pub enum DMXError {
    AlreadyInUse,
    NotValid(DMXErrorValidity),
    NoChannels,
    Other(String) 
}

impl std::fmt::Display for DMXError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DMXError::AlreadyInUse => write!(f, "DMX channel already in use"),
            DMXError::NotValid(exact) => match exact {
                DMXErrorValidity::TooHigh => write!(f, "DMX channel too high"),
                DMXErrorValidity::TooLow => write!(f, "DMX channel too low"),
                // _ => write!(f, "Channel is not valid ( < 1 or > 512"),
            },
            DMXError::NoChannels => write!(f, "No channels available"),
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

#[derive(Debug)]
pub enum DMXErrorValidity {
    TooHigh,
    TooLow,
}