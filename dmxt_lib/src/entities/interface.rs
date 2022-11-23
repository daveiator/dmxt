use super::dmx::Universe;

pub struct Interface {
    id: u32,
    pub name: String,
    pub description: String,
    pub universe: Universe,
    pub model: InterfaceModel,
    
}

// impl Interface {
//     pub fn send() -> Result<(), Error> {
//         Ok(())
//     }
// }

use super::Vendor;

pub struct InterfaceModel {
    pub vendor: Vendor,
    pub name: String,
    pub description: String,
}