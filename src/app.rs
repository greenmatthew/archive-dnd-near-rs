use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::{DiceRoller, Header, RollHistoryPanel, SideNav, TabSystem, TabPanel, TabItem};
use crate::models::RollResult;

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
            <h2>"Welcome"</h2>
            <p>"Select a tool below to get started:"</p>
            
            <div class="tool-links">
                <A href="/dice">"Dice Roller"</A>
                {/* Add more tools here as they are developed */}
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
            id: "dice-roller".to_string(), 
            title: "Standard Roller".to_string() 
        },
        TabItem { 
            id: "custom-calculator".to_string(), 
            title: "Solstora Calculator".to_string() 
        },
    ];
    
    // Track the currently selected tab
    let (selected_tab, set_selected_tab) = create_signal("dice-roller".to_string());
    
    view! {
        <TabSystem 
            tabs=tabs
            selected_tab=selected_tab
            set_selected_tab=set_selected_tab
        >
            // Standard Dice Roller Tab
            <TabPanel tab_id="dice-roller".to_string() selected_tab=selected_tab>
                <StandardDiceRollerContent />
            </TabPanel>
            
            // Solstora Calculator Tab
            <TabPanel tab_id="custom-calculator".to_string() selected_tab=selected_tab>
                <SolstoraCalculatorContent />
            </TabPanel>
        </TabSystem>
    }
}

/// Standard Dice Roller Tab Content
#[component]
fn StandardDiceRollerContent() -> impl IntoView {
    // Signal for the currently selected dice
    let (selected_dice, set_selected_dice) = create_signal(20);
    
    // Signals for dice options
    let (dice_quantity, set_dice_quantity) = create_signal(1);
    let (modifier, set_modifier) = create_signal(0);
    
    // Signals for the result
    let (result, set_result) = create_signal("-".to_string());
    let (result_details, set_result_details) = create_signal("".to_string());
    
    // Signals for custom roll
    let (custom_quantity, set_custom_quantity) = create_signal(1);
    let (custom_dice, set_custom_dice) = create_signal(20);
    let (custom_modifier, set_custom_modifier) = create_signal(0);
    
    // Get the global roll results setter from context
    let set_global_roll_results = use_context::<WriteSignal<Vec<RollResult>>>()
        .expect("set_roll_results should be provided");
    
    // Handler for dice button clicks
    let handle_dice_click = move |dice: u32| {
        set_selected_dice.set(dice);
    };
    
    // Handler for roll button
    let handle_roll = move |_| {
        // Implement the dice rolling logic
        // For now just set dummy values
        set_result.set(format!("15"));
        set_result_details.set(format!("Rolled {}d{}+{}", dice_quantity.get(), selected_dice.get(), modifier.get()));
    };
    
    // Handler for custom roll button
    let handle_custom_roll = move |_| {
        // Implement the custom dice rolling logic
        set_result.set(format!("12"));
        set_result_details.set(format!("Rolled {}d{}+{}", custom_quantity.get(), custom_dice.get(), custom_modifier.get()));
    };
    
    view! {
        <div class="dice-section">
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 4
                on:click=move |_| handle_dice_click(4)
            >
                "D4"
            </button>
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 6
                on:click=move |_| handle_dice_click(6)
            >
                "D6"
            </button>
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 8
                on:click=move |_| handle_dice_click(8)
            >
                "D8"
            </button>
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 10
                on:click=move |_| handle_dice_click(10)
            >
                "D10"
            </button>
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 12
                on:click=move |_| handle_dice_click(12)
            >
                "D12"
            </button>
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 20
                on:click=move |_| handle_dice_click(20)
                style="background-color: rgb(60, 9, 108);"
            >
                "D20"
            </button>
            <button 
                class="dice-button" 
                class:active=move || selected_dice.get() == 100
                on:click=move |_| handle_dice_click(100)
            >
                "D100"
            </button>
        </div>
        
        <div class="modifier-section">
            <label>
                "Number of dice: "
                <input 
                    type="number" 
                    class="quantity-input" 
                    value=dice_quantity.get()
                    min="1" 
                    max="20"
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            if val >= 1 && val <= 20 {
                                set_dice_quantity.set(val);
                            }
                        }
                    }
                />
            </label>
            <label>
                "Modifier: "
                <input 
                    type="number" 
                    class="modifier-input" 
                    value=modifier.get()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            set_modifier.set(val);
                        }
                    }
                />
            </label>
        </div>
        
        <button class="roll-button" on:click=handle_roll>"Roll!"</button>
        
        <div class="result-section">
            <div class="result-display">{move || result.get()}</div>
            <div class="result-details">{move || result_details.get()}</div>
        </div>
        
        <div class="advanced-section">
            <div class="custom-roll-section">
                <input 
                    type="number" 
                    class="quantity-input" 
                    value=custom_quantity.get()
                    min="1" 
                    max="20"
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            if val >= 1 && val <= 20 {
                                set_custom_quantity.set(val);
                            }
                        }
                    }
                />
                <span>"d"</span>
                <input 
                    type="number" 
                    value=custom_dice.get()
                    min="1" 
                    max="1000"
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            if val >= 1 && val <= 1000 {
                                set_custom_dice.set(val);
                            }
                        }
                    }
                />
                <span>"+"</span>
                <input 
                    type="number" 
                    class="modifier-input" 
                    value=custom_modifier.get()
                    on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                            set_custom_modifier.set(val);
                        }
                    }
                />
                <button class="dice-button" on:click=handle_custom_roll>"Roll Custom"</button>
            </div>
        </div>
    }
}

/// Solstora Calculator Tab Content - Basic placeholder
#[component]
fn SolstoraCalculatorContent() -> impl IntoView {
    view! {
        <div>
            <h3>"Solstora Calculator"</h3>
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
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h2>"Not Found"</h2>
        <p>"The page you're looking for doesn't exist."</p>
        <A href="/">"Return to Home"</A>
    }
}