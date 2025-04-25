use leptos::*;
use crate::models::dice::{DiceRoll, DiceRollResult, DiceRollOp};
use std::collections::HashMap;

#[component]
pub fn StandardRoller() -> impl IntoView {
    // Map to store the count of each die type
    let (dice_counts, set_dice_counts) = create_signal::<HashMap<u32, u32>>(HashMap::new());
    
    // Signal for any error message
    let (error_msg, set_error_msg) = create_signal(String::new());
    
    // Local signal to store the most recent roll result
    let (last_roll, set_last_roll) = create_signal::<Option<DiceRollResult>>(None);
    
    // Get the global roll results setter from context
    let set_global_roll_results = use_context::<WriteSignal<Vec<DiceRollResult>>>()
        .expect("set_roll_results should be provided");
    
    // Function to increment a specific die
    let increment_die = move |sides: u32| {
        set_dice_counts.update(|counts| {
            let count = counts.get(&sides).copied().unwrap_or(0);
            counts.insert(sides, count + 1);
        });
    };
    
    // Check if any dice are selected
    let has_selection = move || {
        dice_counts.get().values().any(|&count| count > 0)
    };
    
    // Handle the roll action
    let roll_dice = move |_| {
        set_error_msg.set(String::new()); // Clear any previous error
        
        if !has_selection() {
            set_error_msg.set(String::from("Please select at least one die to roll"));
            return;
        }
        
        // Create all dice rolls based on counts
        let mut all_results = Vec::new();
        let mut combined_dice_results = Vec::new();
        let mut combined_total = 0;
        
        for (&sides, &count) in dice_counts.get().iter() {
            if count > 0 {
                let dice = DiceRoll::create(sides, count, None, Some(0));
                let result = dice.roll();
                
                // Sum up the total
                combined_total += result.result;
                
                // Collect all dice results
                combined_dice_results.extend(result.dice_results.clone());
                
                all_results.push(result);
            }
        }
        
        // Create a combined result
        if !all_results.is_empty() {
            // Create a combined DiceRollResult
            let combined_result = DiceRollResult {
                result: combined_total,
                dice_results: combined_dice_results,
                operation: None,
                modifier: Some(0),
            };
            
            set_last_roll.set(Some(combined_result.clone()));
            
            // Update the global roll history
            set_global_roll_results.update(|results| {
                // Add new result at the beginning of the list
                results.insert(0, combined_result);
                // Keep only the last 10 results
                if results.len() > 10 {
                    results.truncate(10);
                }
            });
            
            // Reset all die counts
            set_dice_counts.set(HashMap::new());
        }
    };
    
    // Function to get the count for a specific die
    let get_die_count = move |sides: u32| -> u32 {
        dice_counts.get().get(&sides).copied().unwrap_or(0)
    };
    
    // Function to check if a die is selected (count > 0)
    let is_die_selected = move |sides: u32| -> bool {
        get_die_count(sides) > 0
    };

    view! {
        <div class="standard-dice-roller">
            // Common dice buttons
            <div class="common-dice">
                <button 
                    on:click=move |_| increment_die(4)
                    class:active=move || is_die_selected(4)
                >
                    "d4"
                    <Show when=move || is_die_selected(4)>
                        <span class="dice-count">{move || get_die_count(4)}</span>
                    </Show>
                </button>
                <button 
                    on:click=move |_| increment_die(6)
                    class:active=move || is_die_selected(6)
                >
                    "d6"
                    <Show when=move || is_die_selected(6)>
                        <span class="dice-count">{move || get_die_count(6)}</span>
                    </Show>
                </button>
                <button 
                    on:click=move |_| increment_die(8)
                    class:active=move || is_die_selected(8)
                >
                    "d8"
                    <Show when=move || is_die_selected(8)>
                        <span class="dice-count">{move || get_die_count(8)}</span>
                    </Show>
                </button>
                <button 
                    on:click=move |_| increment_die(10)
                    class:active=move || is_die_selected(10)
                >
                    "d10"
                    <Show when=move || is_die_selected(10)>
                        <span class="dice-count">{move || get_die_count(10)}</span>
                    </Show>
                </button>
                <button 
                    on:click=move |_| increment_die(12)
                    class:active=move || is_die_selected(12)
                >
                    "d12"
                    <Show when=move || is_die_selected(12)>
                        <span class="dice-count">{move || get_die_count(12)}</span>
                    </Show>
                </button>
                <button 
                    on:click=move |_| increment_die(20)
                    class:active=move || is_die_selected(20)
                >
                    "d20"
                    <Show when=move || is_die_selected(20)>
                        <span class="dice-count">{move || get_die_count(20)}</span>
                    </Show>
                </button>
                <button 
                    on:click=move |_| increment_die(100)
                    class:active=move || is_die_selected(100)
                >
                    "d100"
                    <Show when=move || is_die_selected(100)>
                        <span class="dice-count">{move || get_die_count(100)}</span>
                    </Show>
                </button>
            </div>
            
            // Roll button (now below the dice buttons)
            <div class="roll-button-container">
                <button 
                    on:click=roll_dice
                    class="roll-button"
                    disabled=move || !has_selection()
                >
                    "Roll!"
                </button>
            </div>
            
            // Error message (only shown when there's an error)
            <Show when=move || !error_msg.get().is_empty()>
                <div class="error-message">
                    {move || error_msg.get()}
                </div>
            </Show>
            
            // Always display the last roll container, but with different content based on whether a roll has been made
            <div class="last-roll-container">
                <div class="last-roll-value">
                    {move || last_roll.get().map(|result| result.result.to_string()).unwrap_or_else(|| "-".to_string())}
                </div>
                <div class="last-roll-details">
                    {move || last_roll.get().map(|result| result.to_string()).unwrap_or_default()}
                </div>
            </div>
        </div>
    }
}