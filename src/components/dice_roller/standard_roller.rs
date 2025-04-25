use leptos::*;
use crate::models::dice::{DiceRoll, RollResult, common};

#[component]
pub fn StandardRoller() -> impl IntoView {
    // Signal for the selected dice
    let (selected_dice, set_selected_dice) = create_signal::<Option<DiceRoll>>(None);
    
    // Signal for any error message
    let (error_msg, set_error_msg) = create_signal(String::new());
    
    // Local signal to store the most recent roll result
    let (last_roll, set_last_roll) = create_signal::<Option<RollResult>>(None);
    
    // Get the global roll results setter from context
    let set_global_roll_results = use_context::<WriteSignal<Vec<RollResult>>>()
        .expect("set_roll_results should be provided");
    
    // Handle the roll action
    let roll_dice = move |_| {
        set_error_msg.set(String::new()); // Clear any previous error
        
        if let Some(dice) = selected_dice.get() {
            let result = dice.roll_with_details();
            // Set the local last roll result
            set_last_roll.set(Some(result.clone()));
            // Update the global roll history
            set_global_roll_results.update(|results| {
                // Add new result at the beginning of the list
                results.insert(0, result);
                // Keep only the last 10 results
                if results.len() > 10 {
                    results.truncate(10);
                }
            });
        } else {
            set_error_msg.set(String::from("Please select a die to roll first"));
        }
    };
    
    // Function to handle selecting a die
    let select_die = move |dice: DiceRoll| {
        set_selected_dice.set(Some(dice));
    };

    view! {
        <div class="standard-dice-roller">
            // Common dice buttons
            <div class="common-dice">
                <button 
                    on:click=move |_| select_die(common::d4())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 4 && d.count == 1 && d.modifier == 0)
                >"d4"</button>
                <button 
                    on:click=move |_| select_die(common::d6())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 6 && d.count == 1 && d.modifier == 0)
                >"d6"</button>
                <button 
                    on:click=move |_| select_die(common::d8())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 8 && d.count == 1 && d.modifier == 0)
                >"d8"</button>
                <button 
                    on:click=move |_| select_die(common::d10())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 10 && d.count == 1 && d.modifier == 0)
                >"d10"</button>
                <button 
                    on:click=move |_| select_die(common::d12())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 12 && d.count == 1 && d.modifier == 0)
                >"d12"</button>
                <button 
                    on:click=move |_| select_die(common::d20())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 20 && d.count == 1 && d.modifier == 0)
                >"d20"</button>
                <button 
                    on:click=move |_| select_die(common::d100())
                    class:active=move || selected_dice.get().map_or(false, |d| d.sides == 100 && d.count == 1 && d.modifier == 0)
                >"d100"</button>
            </div>
            
            // Roll button (now below the dice buttons)
            <div class="roll-button-container">
                <button 
                    on:click=roll_dice
                    class="roll-button"
                    disabled=move || selected_dice.get().is_none()
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
                    {move || last_roll.get().map(|result| result.total.to_string()).unwrap_or_else(|| "-".to_string())}
                </div>
                <div class="last-roll-details">
                    {move || last_roll.get().map(|result| result.to_string()).unwrap_or_default()}
                </div>
            </div>
        </div>
    }
}