use leptos::{IntoView, component, create_signal, provide_context, view};
use leptos_meta::{Link, Meta, Stylesheet, Title, provide_meta_context};
use leptos_router::{A, Route, Router, Routes};
use crate::components::{DiceRoller, RollHistoryPanel, SideNav};
use crate::layouts::Header;
use crate::ui::{TabSystem, TabPanel, TabItem};
use crate::models::RollResult;

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
    let (roll_results, set_roll_results) = create_signal::<Vec<RollResult>>(vec![]);
    
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
            <div class="tab-container">
                <div class="tab-buttons">
                    <button class="tab-button active" data-tab="dice-roller">Dice Roller</button>
                    <button class="tab-button" data-tab="custom-calculator">Solstora Calculator</button>
                </div>
            
                <div id="dice-roller" class="tab-content active">
                    <p>Hello</p>
                </div>
            
                <div id="custom-calculator" class="tab-content">
                    <p>World!</p>
                </div>
            </div>
        </div>
    }
}

/// Dice Roller page with tab system
#[component]
fn DiceRollerPage() -> impl IntoView {
    // Create tabs for the dice roller page
    let tabs = vec![
        TabItem { 
            id: "standard".to_string(), 
            title: "Standard Roller".to_string() 
        },
        TabItem { 
            id: "solstora".to_string(), 
            title: "Solstora Calculator".to_string() 
        },
    ];
    
    // Track the currently selected tab
    let (selected_tab, set_selected_tab) = create_signal("standard".to_string());
    
    view! {
        <div class="dice-section-container">
            <TabSystem 
                tabs=tabs
                selected_tab=selected_tab
                set_selected_tab=set_selected_tab
            >
                // Standard Dice Roller Tab
                <TabPanel tab_id="standard".to_string() selected_tab=selected_tab>
                    <SolstoraRollerContent />
                </TabPanel>
                
                // Solstora Calculator Tab - placeholder for now
                <TabPanel tab_id="solstora".to_string() selected_tab=selected_tab>
                    <SolstoraCalculatorContent />
                </TabPanel>
            </TabSystem>
        </div>
    }
}

/// Standard Dice Roller Tab Content
#[component]
fn SolstoraRollerContent() -> impl IntoView {
    view! {
        <div>
            <h3 class="text-center">"Dice Roller"</h3>
            <DiceRoller />
        </div>
    }
}

/// Solstora Calculator Tab Content
#[component]
fn SolstoraCalculatorContent() -> impl IntoView {
    view! {
        <div>
            <h3 class="text-center">"Solstora Calculator"</h3>
            <p>"The Solstora Calculator will be implemented here. It will allow for custom item calculations that D&D Beyond cannot provide."</p>
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