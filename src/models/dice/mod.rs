// Re-export all the public items from each module
pub mod die;
pub mod die_roll_result;
pub mod dice_roll_op;
pub mod dice_roll;
pub mod dice_roll_result;

// Make the important structs and enums directly accessible from the dice module
pub use die::Die;
pub use die_roll_result::DieRollResult;
pub use dice_roll_op::DiceRollOp;
pub use dice_roll::DiceRoll;
pub use dice_roll_result::DiceRollResult;

// Optionally, provide convenience functions at the module level
pub fn parse_dice_expression(expression: &str) -> Result<DiceRoll, String> {
    DiceRoll::from_expression(expression)
}

pub fn roll_dice(expression: &str) -> Result<DiceRollResult, String> {
    let dice_roll = parse_dice_expression(expression)?;
    Ok(dice_roll.roll())
}