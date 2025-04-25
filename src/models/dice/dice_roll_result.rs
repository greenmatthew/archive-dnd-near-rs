use std::fmt;
use super::{DieRollResult, DiceRollOp};

pub struct DiceRollResult {
    result: i32, // Changed to i32 to match with modifier
    dice_results: Vec<DieRollResult>,
    operation: Option<DiceRollOp>,
    modifier: Option<i32>,
}

impl DiceRollResult {
    pub fn create(mut dice_results: Vec<DieRollResult>, operation: Option<DiceRollOp>, modifier: Option<i32>) -> Self {
        // Apply operation (mark dice to keep or discard)
        if let Some(op) = &operation {
            match op {
                DiceRollOp::KeepHighest(n) => {
                    // Sort dice by result (descending)
                    dice_results.sort_by(|a, b| b.result.cmp(&a.result));
                    // Mark dice to keep or discard
                    for (i, die) in dice_results.iter_mut().enumerate() {
                        die.keep = i < *n as usize;
                    }
                },
                DiceRollOp::KeepLowest(n) => {
                    // Sort dice by result (ascending)
                    dice_results.sort_by(|a, b| a.result.cmp(&b.result));
                    // Mark dice to keep or discard
                    for (i, die) in dice_results.iter_mut().enumerate() {
                        die.keep = i < *n as usize;
                    }
                },
            }
        }
        
        // Calculate sum of kept dice
        let sum: i32 = dice_results.iter()
            .filter(|die| die.keep)
            .map(|die| die.result as i32)
            .sum();
        
        // Apply modifier
        let final_result = if let Some(mod_value) = modifier {
            sum + mod_value
        } else {
            sum
        };
        
        Self {
            dice_results,
            operation,
            modifier,
            result: final_result,
        }
    }
}

impl fmt::Display for DiceRollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format individual dice results
        let dice_str: Vec<String> = self.dice_results.iter()
            .map(|die| die.to_string())
            .collect();
        
        // Format operation if present
        let op_str = match &self.operation {
            Some(DiceRollOp::KeepHighest(n)) => format!(" (keeping highest {})", n),
            Some(DiceRollOp::KeepLowest(n)) => format!(" (keeping lowest {})", n),
            None => String::new(),
        };
        
        // Format modifier if present
        let mod_str = match self.modifier {
            Some(m) if m > 0 => format!(" + {}", m),
            Some(m) if m < 0 => format!(" - {}", m.abs()),
            _ => String::new(),
        };
        
        write!(f, "Rolled: {}{}{} = {}", 
            dice_str.join(", "), 
            op_str,
            mod_str,
            self.result
        )
    }
}