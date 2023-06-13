use std::time::Instant;

pub struct Timer<'a> {
    name: &'a str,
    start: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Self {
        Timer {
            name,
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) {
        let elapsed = self.start.elapsed();
        println!("Timer: {}, elapsed: {:?}", self.name, elapsed);
    }
}
