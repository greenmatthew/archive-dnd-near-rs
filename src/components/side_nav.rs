use leptos::*;
use leptos_router::*;

#[component]
pub fn SideNav(show_menu: ReadSignal<bool>, set_show_menu: WriteSignal<bool>) -> impl IntoView {
    let close_menu = move |_| {
        set_show_menu.set(false);
    };

    view! {
        <div class="side-nav-container">
            <div class="side-nav-overlay" 
                 class:visible=move || show_menu.get()
                 on:click=close_menu>
            </div>
            <nav class="side-nav" class:open=move || show_menu.get()>
                <div class="side-nav-header">
                    <h3>"Menu"</h3>
                    <button class="close-button" on:click=close_menu>
                        <span class="material-symbols-outlined">close</span>
                    </button>
                </div>
                <ul class="side-nav-links">
                    <li>
                        <A href="/" exact=true on:click=close_menu>"Home"</A>
                    </li>
                    <li>
                        <A href="/dice" on:click=close_menu>"Dice Roller"</A>
                    </li>
                    // Add more navigation items as you develop more tools
                </ul>
            </nav>
        </div>
    }
}