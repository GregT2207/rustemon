#[derive(Clone)]
pub struct Percentage(u8);

impl Percentage {
    pub fn value(&self) -> u8 {
        return self.0;
    }
}

impl Percentage {
    pub fn new(value: u8) -> Self {
        Percentage(value.clamp(0, 100))
    }
}
