
pub struct DMXUniverse {
    pub channels: [u8; DMX_CHANNELS],
}

pub const DMX_CHANNELS: usize = 512;