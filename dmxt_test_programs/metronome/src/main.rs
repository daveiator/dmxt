use dmxt_lib::timing::Metronome;

use std::sync::Mutex;
use std::sync::Arc;

fn main() {

    let mut counter = Counter::new();

    let mut metronome = Metronome::new(120.0);
    metronome.set_callback(Arc::new(Mutex::new(move || {
        counter.increment();
    }))).unwrap();
    metronome.start();
    println!("Hello, world!");
    //wait 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Goodbye, world!");
    std::thread::sleep(std::time::Duration::from_secs(10));
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
        println!("count: {}", self.count);
    }
}