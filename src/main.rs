use std::sync::{Arc, Mutex};
use std::thread;

struct MyCounter {
    counter: Mutex<i32>,
}

impl MyCounter {
    fn new(initial_value: i32) -> Self {
        Self {
            counter: Mutex::new(initial_value),
        }
    }

    fn increase(self: Arc<Self>, increment: i32) {
        let mut num = self.counter.lock().unwrap();
        *num += increment;
    }

    fn value(self: Arc<Self>) -> i32 {
        *self.counter.lock().unwrap()
    }
}

fn main() {
    let counter = Arc::new(MyCounter::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || counter.increase(1));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.value());
}
