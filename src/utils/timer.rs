use std::time::Instant;

pub struct Timer {
    pub start: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn stop(&self) {
        println!("cost: {:?}", self.start.elapsed());
    }
}
