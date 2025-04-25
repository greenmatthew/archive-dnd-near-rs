// Use this (preferred)
#[derive(Clone)]
pub enum DiceRollOp {
    KeepHighest(u32),
    KeepLowest(u32),
}
