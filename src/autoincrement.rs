pub struct AutoIncrement {
    id: usize,
}

impl AutoIncrement {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn increment(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    pub fn current(&self) -> usize {
        self.id
    }
}

impl Default for AutoIncrement {
    fn default() -> Self {
        Self::new()
    }
}
