use leptos::*;
use leptos_router::*;
use crate::ui::{SlidePanel, SlideDirection};

#[component]
pub fn SideNav(show_menu: ReadSignal<bool>, set_show_menu: WriteSignal<bool>) -> impl IntoView {
    // Create a local handler to ensure the menu closes when links are clicked
    let close_menu = move |_| {
        set_show_menu.set(false);
    };

    view! {
        <SlidePanel 
            title="Menu".to_string()
            is_open=show_menu
            set_open=set_show_menu
            direction=SlideDirection::Left
            width=250
        >
            <ul class="side-nav-links">
                <li>
                    <A href="/" exact=true on:click=close_menu>"Home"</A>
                </li>
                <li>
                    <A href="/dice" on:click=close_menu>"Dice Roller"</A>
                </li>
                // Add more navigation items as you develop more tools
            </ul>
        </SlidePanel>
    }
}