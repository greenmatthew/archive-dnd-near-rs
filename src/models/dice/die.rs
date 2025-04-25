use std::fmt;
use rand::Rng;

#[derive(Clone)]
pub struct Die {
    pub sides: u32
}

impl Die {
    pub fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides)
    }
}

impl fmt::Display for Die {  // This should be Die, not DiceRoll
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "d{}", self.sides)
    }
}