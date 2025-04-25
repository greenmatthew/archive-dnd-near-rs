use leptos::*;

/// Represents a single tab item
#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    /// The unique identifier for the tab
    pub id: String,
    /// The title displayed on the tab
    pub title: String,
}

/// A reusable tab container component
#[component]
pub fn TabContainer(
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
            
            // Render the tab content area
            <div class="tab-content-container">
                {children()}
            </div>
        </div>
    }
}