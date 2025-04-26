use leptos::*;

#[component]
pub fn DieButton(
    sides: u32,
    count: u32,
    on_click: Callback<u32>,
    is_active: bool,
) -> impl IntoView {
    let handle_click = move |_| {
        // Emit the sides value to the parent component
        on_click.call(sides);
    };

    let is_selected = move || count > 0;

    view! {
        <button 
            on:click=handle_click
            class:active=move || is_active
        >
            <span class=format!("die-icon die-icon-d{}", sides)></span>
            <span class="dice-label">{format!("d{}", sides)}</span>
            <Show when=is_selected>
                <span class="dice-count">{count}</span>
            </Show>
        </button>
    }
}