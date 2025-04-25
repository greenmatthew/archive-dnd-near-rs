use leptos::*;
use crate::ui::{TabContainer, TabItem};
use crate::components::dice_roller::{StandardRoller, ExpressionRoller};

#[component]
pub fn DiceRoller() -> impl IntoView {
    // Create tabs for the dice roller page
    let tabs = vec![
        TabItem { 
            id: "standard".to_string(), 
            title: "Standard Roller".to_string() 
        },
        TabItem { 
            id: "solstora".to_string(), 
            title: "Solstora Calculator".to_string() 
        },
    ];
    
    // Track the currently selected tab
    let (selected_tab, set_selected_tab) = create_signal("standard".to_string());
    
    view! {
        <div class="dice-section-container">
            <TabContainer 
                tabs=tabs
                selected_tab=selected_tab
                set_selected_tab=set_selected_tab
            >
                // We'll use the Show component from Leptos to conditionally render content
                // based on the selected tab
                <Show when=move || selected_tab.get() == "standard">
                    <div class="tab-content active">
                        <StandardRoller />
                    </div>
                </Show>
                
                <Show when=move || selected_tab.get() == "solstora">
                    <div class="tab-content active">
                        <ExpressionRoller />
                    </div>
                </Show>
            </TabContainer>
        </div>
    }
}
