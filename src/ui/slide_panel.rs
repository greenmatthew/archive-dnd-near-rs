use leptos::*;

// Direction the panel will slide from
#[derive(Clone, Copy, PartialEq)]
pub enum SlideDirection {
    Left,
    Right,
}

// We'll keep the component function approach and remove the struct
#[component]
pub fn SlidePanel(
    children: Children,
    title: String,
    is_open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    #[prop(default = SlideDirection::Left)] direction: SlideDirection,
    #[prop(default = 250)] width: u32,
) -> impl IntoView {
    let close_panel = move |_| {
        set_open.set(false);
    };

    // Get the class and toggle position based on direction
    let panel_class = match direction {
        SlideDirection::Left => "slide-panel-left",
        SlideDirection::Right => "slide-panel-right",
    };

    view! {
        <div class="slide-panel-container">
            <div 
                class="slide-panel-overlay" 
                class:visible=move || is_open.get()
                on:click=close_panel
            >
            </div>
            <div 
                class=format!("slide-panel {}", panel_class)
                class:open=move || is_open.get()
                style=format!("width: {}px", width)
            >
                <div class="panel-header">
                    <h3>{title}</h3>
                    <button class="close-button" on:click=close_panel>
                        <span class="material-symbols-outlined">close</span>
                    </button>
                </div>
                
                <div class="panel-content">
                    {children()}
                </div>
            </div>
        </div>
    }
}