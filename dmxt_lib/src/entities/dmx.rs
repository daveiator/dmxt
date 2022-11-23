//Lowest level

pub struct Universe {
    id: u32,
    pub name: String,
    pub description: String,
    pub channels: Vec<Channel>,
}

pub struct Channel {
    id: u8,
    value: u8,
}