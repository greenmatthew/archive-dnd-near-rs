use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="app-header">
            <div class="header-content">
                <div class="logo">
                    <A href="/">
                        <img src="/assets/images/logo.svg" alt="D&D Helper Logo" />
                    </A>
                </div>
                <nav class="main-nav">
                    <ul>
                        <li><A href="/" exact=true>"Home"</A></li>
                        <li><A href="/dice">"Dice Roller"</A></li>
                        // Add more navigation items as you develop more tools
                    </ul>
                </nav>
            </div>
        </header>
    }
}