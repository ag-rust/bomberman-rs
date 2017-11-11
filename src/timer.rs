use std::time::Instant;

pub struct Timer {
    start: Instant,
    name: Option<&'static str>,
}

impl Timer {
    pub fn start() -> Timer {
        Timer {
            start: Instant::now(),
            name: None,
        }
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = Some(name);
    }

    pub fn print_duration(&self) {
        if let Some(name) = self.name {
            println!("{}: {:?}", name, self.start.elapsed());
        } else {
            println!("{:?}", self.start.elapsed());
        }
    }
}
