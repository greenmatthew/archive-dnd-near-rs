use leptos::*;
use crate::models::dice::{DiceRoll, RollResult, common};

#[component]
pub fn StandardRoller() -> impl IntoView {
    // Signal for the dice notation input
    let (dice_notation, set_dice_notation) = create_signal(String::from("2d6"));
    
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
        
        match DiceRoll::from_notation(&dice_notation.get()) {
            Some(dice) => {
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
            },
            None => {
                set_error_msg.set(format!("Invalid dice notation: {}", dice_notation.get()));
            }
        }
    };
    
    // Common dice buttons
    let roll_common = move |dice: DiceRoll| {
        set_dice_notation.set(dice.to_string());
        let result = dice.roll_with_details();
        // Set the local last roll result
        set_last_roll.set(Some(result.clone()));
        // Update the global roll history
        set_global_roll_results.update(|results| {
            results.insert(0, result);
            if results.len() > 10 {
                results.truncate(10);
            }
        });
    };

    view! {
        <div class="standard-dice-roller">
            <div class="input-group">
                <input 
                    type="text" 
                    prop:value=move || dice_notation.get()
                    on:input=move |ev| {
                        set_dice_notation.set(event_target_value(&ev));
                    }
                    placeholder="e.g., 2d6+3"
                />
                <button on:click=roll_dice>"Roll!"</button>
            </div>
            
            // Error message (only shown when there's an error)
            <Show when=move || !error_msg.get().is_empty()>
                <div class="error-message">
                    {move || error_msg.get()}
                </div>
            </Show>
            
            // Common dice buttons
            <div class="common-dice">
                <button on:click=move |_| roll_common(common::d4())>"d4"</button>
                <button on:click=move |_| roll_common(common::d6())>"d6"</button>
                <button on:click=move |_| roll_common(common::d8())>"d8"</button>
                <button on:click=move |_| roll_common(common::d10())>"d10"</button>
                <button on:click=move |_| roll_common(common::d12())>"d12"</button>
                <button on:click=move |_| roll_common(common::d20())>"d20"</button>
                <button on:click=move |_| roll_common(common::d100())>"d100"</button>
            </div>
            
            // Display the last roll result
            <Show when=move || last_roll.get().is_some()>
                <div class="last-roll-container">
                    <h3>"Last Roll"</h3>
                    <div class="last-roll-result">
                        {move || last_roll.get().map(|result| result.to_string()).unwrap_or_default()}
                    </div>
                </div>
            </Show>
        </div>
    }
}
