use leptos::*;
use leptos_router::*;

#[component]
pub fn Header(
    show_menu: WriteSignal<bool>,
    show_roll_history: WriteSignal<bool>
) -> impl IntoView {
    let toggle_menu = move |_| {
        show_menu.update(|show| *show = !*show);
    };

    let toggle_roll_history = move |_| {
        show_roll_history.update(|show| *show = !*show);
    };

    view! {
        <header class="app-header">
            <div class="header-content">
                <button class="hamburger-menu" on:click=toggle_menu>
                    <span class="material-symbols">menu</span>
                </button>
                <div class="logo">
                    <A href="/">
                        <img src="/assets/images/logo.svg" alt="D&D Helper Logo" />
                    </A>
                </div>
                <button class="roll-history" on:click=toggle_roll_history>
                    <span class="material-symbols-outlined">deployed_code_history</span>
                </button>
            </div>
        </header>
    }
}