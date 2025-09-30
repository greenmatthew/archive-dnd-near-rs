// Use this (preferred)
#[derive(Clone, Debug)]
pub enum DiceRollOp {
    KeepHighest(u32),
    KeepLowest(u32),
}
