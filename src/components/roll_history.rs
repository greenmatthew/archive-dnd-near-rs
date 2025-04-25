use leptos::*;
use crate::ui::{SlidePanel, SlideDirection};
use crate::models::dice::DiceRollResult; // Updated import path

#[component]
pub fn RollHistoryPanel(
    roll_results: ReadSignal<Vec<DiceRollResult>>,
    is_open: ReadSignal<bool>,
    set_open: WriteSignal<bool>
) -> impl IntoView {
    view! {
        <SlidePanel 
            title="Roll History".to_string()
            is_open=is_open
            set_open=set_open
            direction=SlideDirection::Right
            width=300
        >
            {move || {
                let results = roll_results.get();
                if results.is_empty() {
                    view! {
                        <div class="empty-message">
                            "No dice rolls yet. Use the dice roller to see your roll history."
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <For
                            each=move || roll_results.get()
                            key=|result| format!("{:?}-{}", result.dice_results, result.result) // This needs to be updated
                            let:result
                        >
                            <div class="roll-result">
                                {move || result.to_string()}
                            </div>
                        </For>
                    }.into_view()
                }
            }}
        </SlidePanel>
    }
}