use leptos::{IntoView, component, create_signal, provide_context, view};
use leptos_meta::{Link, Meta, Stylesheet, Title, provide_meta_context};
use leptos_router::{A, Route, Router, Routes};
use crate::components::{DiceRoller, RollHistoryPanel, SideNav};
use crate::layouts::Header;
use crate::models::DiceRollResult;

#[must_use]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    // Signal to track if the menu is open or closed
    let (show_menu, set_show_menu) = create_signal(false);
    
    // Signal to track if the roll history panel is open or closed
    let (show_roll_history, set_show_roll_history) = create_signal(false);
    
    // Create a resource to store roll results globally
    let (roll_results, set_roll_results) = create_signal::<Vec<DiceRollResult>>(vec![]);
    
    // Create context for roll results so any component can access it
    provide_context(set_roll_results);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dnd-near-rs.css"/>
        
        <Stylesheet id="material-symbols-outlined" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0"/>
        <Stylesheet id="material-symbols" href="https://fonts.googleapis.com/css2?family=Material+Symbols:opsz,wght,FILL,GRAD@24,400,1,0"/>

        <link rel="stylesheet" href="&icon_names=home" />

        <Link rel="icon" type_="image/x-icon" href="/assets/favicon.ico"/>
        <Link rel="apple-touch-icon" sizes="180x180" href="/assets/apple-touch-icon.png"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/assets/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/assets/favicon-16x16.png"/>
        <Link rel="manifest" href="/assets/site.webmanifest"/>

        <Meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no, viewport-fit=cover"/>
        <Meta name="apple-mobile-web-app-capable" content="yes"/>
        <Meta name="apple-mobile-web-app-status-bar-style" content="black-translucent"/>

        <Title text="D&D Near"/>

        <Router>
            <Header 
                show_menu=set_show_menu 
                show_roll_history=set_show_roll_history 
            />
            <SideNav 
                show_menu=show_menu 
                set_show_menu=set_show_menu 
            />
            <main class="app-content">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/dice" view=DiceRollerPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            <RollHistoryPanel 
                roll_results=roll_results
                is_open=show_roll_history 
                set_open=set_show_roll_history 
            />
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h2>"Welcome to D&D Near"</h2>
            <p>"A simple toolkit to help with your D&D sessions."</p>
            <div class="tool-links">
                <A href="/dice">"Dice Roller"</A>
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
        let resp = leptos::expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h2>"Not Found"</h2>
        <p>"The page you're looking for doesn't exist."</p>
        <A href="/">"Return to Home"</A>
    }
}