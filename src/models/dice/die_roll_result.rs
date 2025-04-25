use std::fmt;
use super::Die;

#[derive(Clone)]
pub struct DieRollResult {
    pub die: Die,
    pub result: u32,
    pub keep: bool,
}

impl fmt::Display for DieRollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.keep {
            write!(f, "[{}]", self.result)
        } else {
            write!(f, "({}: discarded)", self.result)
        }
    }
}