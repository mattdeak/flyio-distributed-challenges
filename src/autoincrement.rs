use std::sync::Mutex;

pub struct AutoIncrement {
    id: Mutex<usize>,
}

impl AutoIncrement {
    pub fn new() -> Self {
        Self { id: Mutex::new(0) }
    }

    pub fn increment(&self) -> usize {
        let mut id = self.id.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn current(&self) -> usize {
        *self.id.lock().unwrap()
    }
}

impl Default for AutoIncrement {
    fn default() -> Self {
        Self::new()
    }
}
