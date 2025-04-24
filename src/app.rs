use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::DiceRoller;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dnd-near-rs.css"/>

        // sets the document title
        <Title text="D&D Helper"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/dice" view=DiceRollerPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"D&D Helper"</h1>
            <p>"Welcome to your D&D Helper application. Select a tool below:"</p>
            
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
            <h1>"D&D Helper - Dice Roller"</h1>
            <A href="/">"Back to Home"</A>
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
        <h1>"Not Found"</h1>
        <p>"The page you're looking for doesn't exist."</p>
        <A href="/">"Return to Home"</A>
    }
}