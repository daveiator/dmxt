pub mod entities;
pub mod dmx_serial;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn check_imports() {
        use crate::entities::dmx::Universe;
        use crate::entities::interface::Interface;
        use crate::dmx_serial::DMXSerial;
    }
}
