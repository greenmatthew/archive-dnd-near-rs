use leptos::*;

/// Represents a single tab item
#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    /// The unique identifier for the tab
    pub id: String,
    /// The title displayed on the tab
    pub title: String,
}

/// A reusable tab component
#[component]
pub fn TabSystem(
    /// List of tab items
    tabs: Vec<TabItem>,
    /// Signal for the currently selected tab
    selected_tab: ReadSignal<String>,
    /// Signal to set the currently selected tab
    set_selected_tab: WriteSignal<String>,
    /// Children components to render inside the tab content area
    children: Children,
) -> impl IntoView {
    // Function to handle tab selection
    let handle_tab_click = move |tab_id: String| {
        set_selected_tab.set(tab_id);
    };

    view! {
        <div class="tab-container">
            <div class="tab-buttons">
                {tabs.into_iter().map(move |tab| {
                    let tab_id_for_active = tab.id.clone();
                    let tab_id_for_click = tab.id.clone();
                    let tab_title = tab.title;
                    
                    let is_active = move || selected_tab.get() == tab_id_for_active;
                    
                    view! {
                        <button 
                            class="tab-button" 
                            class:active=is_active
                            on:click=move |_| handle_tab_click(tab_id_for_click.clone())
                        >
                            {tab_title}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
            {children()}
        </div>
    }
}

/// Component to wrap content for a specific tab
#[component]
pub fn TabPanel(
    /// The unique identifier for this tab panel, must match a tab id
    tab_id: String,
    /// Signal for the currently selected tab
    selected_tab: ReadSignal<String>,
    /// Children components within this tab panel
    children: Children,
) -> impl IntoView {
    // Clone the tab_id before it gets moved into the closure
    let tab_id_clone = tab_id.clone();
    
    let is_active = move || selected_tab.get() == tab_id_clone;
    
    view! {
        <div id={tab_id} class="tab-content" class:active=is_active>
            {children()}
        </div>
    }
}