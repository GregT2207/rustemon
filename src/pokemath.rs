#[derive(Clone, Debug)]
pub struct Percentage(u8);

impl Percentage {
    pub fn value(&self) -> u8 {
        return self.0;
    }
}

impl Percentage {
    pub fn new(value: u8) -> Self {
        Percentage(value.clamp(0, 100) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_within_acceptable_range_remains_the_same() {
        let value = 48;
        let percentage = Percentage::new(value);

        assert_eq!(percentage.value(), value as u8);
    }

    #[test]
    fn value_above_100_clamps_to_100() {
        let value = 204;
        let percentage = Percentage::new(value);

        assert_eq!(percentage.value(), 100);
    }
}
