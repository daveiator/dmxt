use dmxt_lib::timing::Metronome;

use std::sync::Mutex;
use std::sync::Arc;

use std::io::{stdin,stdout,Write};

fn main() {
    unsafe {

        static mut COUNTER: Counter = Counter{count: 0};

        let callback_1 = Arc::new(Mutex::new(|| {
            COUNTER.increment();
        }));

        let _callback_2 = Arc::new(Mutex::new(|| {
            COUNTER.decrement();
        }));


        let mut metronome = Metronome::new(120.0);
        metronome.set_callback(callback_1).unwrap();
        println!("Starting with callback 1");
        metronome.start().unwrap();

        let mut s = String::new();
        loop {
            let _=stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            metronome.tap();
            println!("BPM: {}", metronome.get_bpm());
        }
    }
}

struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
        println!("count: {}", self.count);
    }
    
    fn decrement(&mut self) {
        self.count -= 1;
        println!("count: {}", self.count);
    }
}