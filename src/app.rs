use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::{DiceRoller, Header, RollHistoryPanel, SideNav};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    // Signal to track if the menu is open or closed
    let (show_menu, set_show_menu) = create_signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dnd-near-rs.css"/>

        <Stylesheet href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0&icon_names=close,menu"/>

        <Link rel="icon" type_="image/x-icon" href="/assets/favicon.ico"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/assets/apple-touch-icon.png"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/assets/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/assets/favicon-16x16.png"/>
        <Link rel="manifest" href="/assets/site.webmanifest"/>

        <Title text="D&D Near"/>

        <Router>
            <Header show_menu=set_show_menu />
            <SideNav show_menu=show_menu set_show_menu=set_show_menu />
            <main class="app-content">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/dice" view=DiceRollerPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            <RollHistoryPanel />
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h2>"Welcome"</h2>
            <p>"Select a tool below to get started:"</p>
            
            <div class="tool-links">
                <A href="/dice">"Dice Roller"</A>
                {/* Add more tools here as they are developed */}
            </div>
        </div>
    }
}

/// Dice Roller page
#[component]
fn DiceRollerPage() -> impl IntoView {
    view! {
        <div>
            <h2>"Dice Roller"</h2>
            <DiceRoller />
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h2>"Not Found"</h2>
        <p>"The page you're looking for doesn't exist."</p>
        <A href="/">"Return to Home"</A>
    }
}