pub struct FrameCounter {
    count: i64,
}

pub enum IncrementResult {
    Ok,
    Overflow,
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter { count: 0 }
    }

    pub fn increment(&mut self) -> IncrementResult {
        if let Some(new_count) = self.count.checked_add(1) {
            self.count = new_count;
            IncrementResult::Ok
        } else {
            IncrementResult::Overflow
        }
    }
}
