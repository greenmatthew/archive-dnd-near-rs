use leptos::*;
use crate::models::dice::{DiceRoll, RollResult, common};

#[component]
pub fn DiceRoller() -> impl IntoView {
    // Signal for the dice notation input
    let (dice_notation, set_dice_notation) = create_signal(String::from("2d6"));
    
    // Signal for storing roll results
    let (roll_results, set_roll_results) = create_signal::<Vec<RollResult>>(vec![]);
    
    // Signal for any error message
    let (error_msg, set_error_msg) = create_signal(String::new());
    
    // Handle the roll action
    let roll_dice = move |_| {
        set_error_msg.set(String::new()); // Clear any previous error
        
        match DiceRoll::from_notation(&dice_notation.get()) {
            Some(dice) => {
                let result = dice.roll_with_details();
                set_roll_results.update(|results| {
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
        set_roll_results.update(|results| {
            results.insert(0, result);
            if results.len() > 10 {
                results.truncate(10);
            }
        });
    };

    view! {
        <div class="dice-roller">
            <h2>"Dice Roller"</h2>
            
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
            
            // Roll history
            <div class="roll-history">
                <h3>"Roll History"</h3>
                <For
                    each=move || roll_results.get()
                    key=|result| format!("{:?}-{}", result.individual_rolls, result.total)
                    let:result
                >
                    <div class="roll-result">
                        {move || result.to_string()}
                    </div>
                </For>
            </div>
        </div>
    }
}