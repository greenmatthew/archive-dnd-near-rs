use std::fmt;
use regex::Regex;
use super::{Die, DieRollResult, DiceRollResult, DiceRollOp};


pub struct DiceRoll {
    pub die: Die,
    pub die_count: u32,
    pub operation: Option<DiceRollOp>,
    pub modifier: Option<i32>,
}

impl DiceRoll {
    pub fn create(sides: u32, die_count: u32, operation: Option<DiceRollOp>, modifier: Option<i32>) -> Self {
        assert!(sides > 0, "Die must have at least 1 side");
        assert!(die_count > 0, "Must roll at least 1 die");
        
        // Check that operation values are valid if present
        if let Some(op) = &operation {
            match op {
                DiceRollOp::KeepHighest(n) => assert!(*n > 0, "KeepHighest must be at least 1"),
                DiceRollOp::KeepLowest(n) => assert!(*n > 0, "KeepLowest must be at least 1"),
            }
        }
        
        Self {
            die: Die { sides },
            die_count,
            operation,
            modifier,
        }
    }

    pub fn from_expression(exp: &str) -> Result<Self, String> {
        let exp = exp.trim();
        // This regex captures: 
        // 1. die count
        // 2. die sides
        // 3. optional keep operation (kh or kl) with its value
        // 4. optional modifier with sign
        let re = Regex::new(r"^(\d+)d(\d+)(?:(kh|kl)(\d+))?(?:([+-])(\d+))?$").map_err(|e| e.to_string())?;
        
        if let Some(caps) = re.captures(exp) {
            let die_count = caps.get(1).unwrap().as_str().parse::<u32>()
                .map_err(|_| "Invalid die count".to_string())?;
                
            let sides = caps.get(2).unwrap().as_str().parse::<u32>()
                .map_err(|_| "Invalid die sides".to_string())?;
                
            // Parse operation if present
            let operation = if caps.get(3).is_some() && caps.get(4).is_some() {
                let op_type = caps.get(3).unwrap().as_str();
                let op_value = caps.get(4).unwrap().as_str().parse::<u32>()
                    .map_err(|_| "Invalid operation value".to_string())?;
                    
                match op_type {
                    "kh" => Some(DiceRollOp::KeepHighest(op_value)),
                    "kl" => Some(DiceRollOp::KeepLowest(op_value)),
                    _ => None, // This shouldn't happen due to regex constraint
                }
            } else {
                None
            };
            
            // Parse modifier if present
            let modifier = if caps.get(5).is_some() && caps.get(6).is_some() {
                let sign = caps.get(5).unwrap().as_str();
                let value = caps.get(6).unwrap().as_str().parse::<i32>()
                    .map_err(|_| "Invalid modifier value".to_string())?;
                    
                match sign {
                    "+" => Some(value),
                    "-" => Some(-value),
                    _ => None, // This shouldn't happen due to regex constraint
                }
            } else {
                None
            };
            
            // Use create to enforce validation
            Ok(Self::create(sides, die_count, operation, modifier))
        } else {
            Err(format!("Failed to parse dice expression: {}", exp))
        }
    }

    pub fn roll(&self) -> DiceRollResult {
        let die_results: Vec<DieRollResult> = (0..self.die_count)
            .map(|_| {
                let value = self.die.roll();
                DieRollResult {
                    die: Die { sides: self.die.sides },
                    result: value,
                    keep: true, // Initially mark all as kept
                }
            })
            .collect();
        
        DiceRollResult::create(die_results, self.operation.clone(), self.modifier.clone())
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match &self.operation {
            Some(DiceRollOp::KeepHighest(n)) => format!("kh{}", n),
            Some(DiceRollOp::KeepLowest(n)) => format!("kl{}", n),
            None => String::new(),
        };
        
        let modifier = match self.modifier {
            Some(m) if m > 0 => format!("+{}", m),
            Some(m) if m < 0 => format!("-{}", m.abs()),
            _ => String::new(),
        };
        
        write!(f, "{}{}{}{}", self.die_count, self.die, op, modifier)
    }
}