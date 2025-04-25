use rand::Rng;
use std::fmt;

/// Represents a standard dice notation (e.g., "2d6+3")
#[derive(Debug, Clone)]
pub struct DiceRoll {
    /// Number of dice to roll
    pub count: u32,
    /// Number of sides on each die
    pub sides: u32,
    /// Modifier to add to the result
    pub modifier: i32,
}

impl DiceRoll {
    /// Create a new dice roll with the specified parameters
    pub fn new(count: u32, sides: u32, modifier: i32) -> Self {
        Self {
            count,
            sides,
            modifier,
        }
    }

    /// Roll the dice and return the total
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let dice_total: u32 = (0..self.count)
            .map(|_| rng.gen_range(1..=self.sides))
            .sum();
        
        dice_total as i32 + self.modifier
    }

    /// Roll the dice and return detailed information about each die rolled
    pub fn roll_with_details(&self) -> RollResult {
        let mut rng = rand::thread_rng();
        let individual_rolls: Vec<u32> = (0..self.count)
            .map(|_| rng.gen_range(1..=self.sides))
            .collect();
        
        let total: u32 = individual_rolls.iter().sum();
        
        RollResult {
            dice_roll: self.clone(),
            individual_rolls,
            total: total as i32 + self.modifier,
        }
    }
    
    /// Parse a string in dice notation format (e.g., "2d6+3")
    /// Returns None if the string is not in a valid format
    pub fn from_notation(notation: &str) -> Option<Self> {
        // Remove all whitespace
        let notation = notation.replace(' ', "");
        
        // Check for basic format
        if !notation.contains('d') {
            return None;
        }
        
        // Split into parts
        let parts: Vec<&str> = notation.split('d').collect();
        if parts.len() != 2 {
            return None;
        }
        
        // Parse count (number of dice)
        let count = if parts[0].is_empty() {
            1 // Default to 1 if no count specified (e.g., "d20")
        } else {
            match parts[0].parse::<u32>() {
                Ok(n) => n,
                Err(_) => return None,
            }
        };
        
        // Split sides and modifier
        let mut modifier = 0;
        let sides_part = parts[1];
        
        // Check for modifier
        let sides = if sides_part.contains('+') {
            let mod_parts: Vec<&str> = sides_part.split('+').collect();
            if mod_parts.len() != 2 {
                return None;
            }
            
            match mod_parts[1].parse::<i32>() {
                Ok(m) => modifier = m,
                Err(_) => return None,
            }
            
            match mod_parts[0].parse::<u32>() {
                Ok(s) => s,
                Err(_) => return None,
            }
        } else if sides_part.contains('-') {
            let mod_parts: Vec<&str> = sides_part.split('-').collect();
            if mod_parts.len() != 2 {
                return None;
            }
            
            match mod_parts[1].parse::<i32>() {
                Ok(m) => modifier = -m, // Note the negative sign
                Err(_) => return None,
            }
            
            match mod_parts[0].parse::<u32>() {
                Ok(s) => s,
                Err(_) => return None,
            }
        } else {
            // No modifier
            match sides_part.parse::<u32>() {
                Ok(s) => s,
                Err(_) => return None,
            }
        };
        
        Some(Self {
            count,
            sides,
            modifier,
        })
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)?;
        
        match self.modifier.cmp(&0) {
            std::cmp::Ordering::Greater => write!(f, "+{}", self.modifier),
            std::cmp::Ordering::Less => write!(f, "{}", self.modifier), // The negative sign is included in the number
            std::cmp::Ordering::Equal => Ok(()),
        }
    }
}

/// Represents the result of a dice roll with detailed information
#[derive(Debug, Clone)]
pub struct RollResult {
    /// The original dice roll information
    pub dice_roll: DiceRoll,
    /// Individual values of each die rolled
    pub individual_rolls: Vec<u32>,
    /// Total result including modifier
    pub total: i32,
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → [", self.dice_roll)?;
        
        for (i, roll) in self.individual_rolls.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{roll}")?;
        }
        
        write!(f, "]")?;
        
        if self.dice_roll.modifier != 0 {
            if self.dice_roll.modifier > 0 {
                write!(f, " + {}", self.dice_roll.modifier)?;
            } else {
                write!(f, " - {}", -self.dice_roll.modifier)?;
            }
        }
        
        write!(f, " = {}", self.total)
    }
}

/// Predefined common D&D dice values
pub mod common {
    use super::DiceRoll;
    
    /// Returns a d4 dice roll (1d4)
    pub fn d4() -> DiceRoll {
        DiceRoll::new(1, 4, 0)
    }
    
    /// Returns a d6 dice roll (1d6)
    pub fn d6() -> DiceRoll {
        DiceRoll::new(1, 6, 0)
    }
    
    /// Returns a d8 dice roll (1d8)
    pub fn d8() -> DiceRoll {
        DiceRoll::new(1, 8, 0)
    }
    
    /// Returns a d10 dice roll (1d10)
    pub fn d10() -> DiceRoll {
        DiceRoll::new(1, 10, 0)
    }
    
    /// Returns a d12 dice roll (1d12)
    pub fn d12() -> DiceRoll {
        DiceRoll::new(1, 12, 0)
    }
    
    /// Returns a d20 dice roll (1d20)
    pub fn d20() -> DiceRoll {
        DiceRoll::new(1, 20, 0)
    }
    
    /// Returns a d100 dice roll (1d100)
    pub fn d100() -> DiceRoll {
        DiceRoll::new(1, 100, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dice_creation() {
        let dice = DiceRoll::new(2, 6, 3);
        assert_eq!(dice.count, 2);
        assert_eq!(dice.sides, 6);
        assert_eq!(dice.modifier, 3);
    }
    
    #[test]
    fn test_dice_display() {
        let dice1 = DiceRoll::new(2, 6, 3);
        assert_eq!(dice1.to_string(), "2d6+3");
        
        let dice2 = DiceRoll::new(1, 20, -2);
        assert_eq!(dice2.to_string(), "1d20-2");
        
        let dice3 = DiceRoll::new(3, 8, 0);
        assert_eq!(dice3.to_string(), "3d8");
    }
    
    #[test]
    fn test_dice_notation_parsing() {
        // Test basic notation
        let dice1 = DiceRoll::from_notation("2d6+3").unwrap();
        assert_eq!(dice1.count, 2);
        assert_eq!(dice1.sides, 6);
        assert_eq!(dice1.modifier, 3);
        
        // Test with spaces
        let dice2 = DiceRoll::from_notation("2 d 6 + 3").unwrap();
        assert_eq!(dice2.count, 2);
        assert_eq!(dice2.sides, 6);
        assert_eq!(dice2.modifier, 3);
        
        // Test negative modifier
        let dice3 = DiceRoll::from_notation("1d20-2").unwrap();
        assert_eq!(dice3.count, 1);
        assert_eq!(dice3.sides, 20);
        assert_eq!(dice3.modifier, -2);
        
        // Test no modifier
        let dice4 = DiceRoll::from_notation("3d8").unwrap();
        assert_eq!(dice4.count, 3);
        assert_eq!(dice4.sides, 8);
        assert_eq!(dice4.modifier, 0);
        
        // Test implicit count (d20 is equivalent to 1d20)
        let dice5 = DiceRoll::from_notation("d20").unwrap();
        assert_eq!(dice5.count, 1);
        assert_eq!(dice5.sides, 20);
        assert_eq!(dice5.modifier, 0);
    }
}