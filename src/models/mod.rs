pub mod damage;
pub mod dice;
pub mod roll_history;

// Re-export common types
pub use damage::{DamageType, Damage};
pub use dice::{DiceRoll, DiceRollResult};
pub use roll_history::{DiceHistoryEntry, DiceHistoryStore};