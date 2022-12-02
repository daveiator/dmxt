use dmxt_lib::dmx::serial::DMXSerialAgent;
use std::thread;
fn main() {
    let mut dmx = DMXSerialAgent::init("COM4").unwrap();
    loop {
        dmx.send_dmx_packet([0; 512]).unwrap();
        dmx.send_dmx_packet([255; 512]).unwrap();

    }
}
