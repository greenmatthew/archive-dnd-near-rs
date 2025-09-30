use leptos::*;
use crate::ui::{SlidePanel, SlideDirection};
use crate::models::roll_history::use_dice_history;

#[component]
pub fn RollHistoryPanel(
    is_open: ReadSignal<bool>,
    set_open: WriteSignal<bool>
) -> impl IntoView {
    // Get the history store from context
    let history_store = use_dice_history();
    let history = history_store.get_history();
    
    view! {
        <SlidePanel 
            title="Roll History".to_string()
            is_open=is_open
            set_open=set_open
            direction=SlideDirection::Right
            width=300
        >
            <div class="roll-history-container">
                <button 
                    class="clear-history-btn"
                    on:click=move |_| history_store.clear()
                >
                    "Clear History"
                </button>
                
                {move || {
                    let entries = history.get();
                    if entries.is_empty() {
                        view! {
                            <div class="empty-message">
                                "No dice rolls yet. Use the dice roller to see your roll history."
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <For
                                each=move || history.get()
                                key=|entry| format!("{:?}", entry.timestamp.timestamp_millis())
                                let:entry
                            >
                                <div class="roll-result">
                                    <span class="roll-time">{entry.timestamp.format("%H:%M:%S").to_string()}</span>
                                    <span class="roll-details">
                                        {move || {
                                            entry.roll_results.iter()
                                                .map(|r| r.to_string())
                                                .collect::<Vec<_>>()
                                                .join(", ")
                                        }}
                                    </span>
                                </div>
                            </For>
                        }.into_view()
                    }
                }}
            </div>
        </SlidePanel>
    }
}