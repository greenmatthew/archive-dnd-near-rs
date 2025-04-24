pub mod damage;
pub mod dice;

// Re-export common types
pub use damage::{DamageType, Damage};
pub use dice::{DiceRoll, RollResult};