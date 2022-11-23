use dmxt_lib::dmx_serial::DMXSerial;
use std::thread::sleep;

fn main() {
    let mut dmx = DMXSerial::open("COM4").unwrap();
        dmx.set_channel(1, 255).unwrap();
        sleep(std::time::Duration::from_millis(1000));
        dmx.set_channel(512, 255).unwrap();
        sleep(std::time::Duration::from_millis(1000));
}