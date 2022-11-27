use dmxt_lib::timing::Metronome;

use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    unsafe {

        static mut counter: Counter = Counter{count: 0};

        let callback_1 = Arc::new(Mutex::new(|| {
            counter.increment();
        }));

        let callback_2 = Arc::new(Mutex::new(|| {
            counter.decrement();
        }));


        let mut metronome = Metronome::new(120.0);
        metronome.set_callback(callback_1).unwrap();
        println!("Starting with callback 1");
        metronome.start();
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("Switching to callback 2");
        metronome.set_callback(callback_2).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));

    }
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
        println!("count: {}", self.count);
    }
    
    fn decrement(&mut self) {
        self.count -= 1;
        println!("count: {}", self.count);
    }
}