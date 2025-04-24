use leptos::*;

#[component]
pub fn RollHistoryPanel() -> impl IntoView {
    // Signal to track if the panel is open or closed
    let (is_open, set_is_open) = create_signal(false);
    
    // Toggle panel visibility
    let toggle_panel = move |_| {
        set_is_open.update(|open| *open = !*open);
    };
    
    // Close the panel
    let close_panel = move |_| {
        set_is_open.set(false);
    };
    
    view! {
        <div class="roll-history-panel-container">
            <div 
                class="roll-history-panel" 
                class:open=move || is_open.get()
            >
                // Toggle button (arrow) - now attached to the panel
                <button 
                    class="panel-toggle"
                    on:click=toggle_panel
                >
                    <span class="toggle-arrow">
                        {move || if is_open.get() { "›" } else { "‹" }}
                    </span>
                </button>
                
                <div class="panel-header">
                    <h3>"Roll History"</h3>
                    <button class="close-button" on:click=close_panel>"×"</button>
                </div>
                
                <div class="panel-content">
                    // Panel is empty by design - will be filled later
                    <div class="empty-message">
                        "Roll history panel (empty by design)"
                    </div>
                </div>
            </div>
        </div>
    }
}