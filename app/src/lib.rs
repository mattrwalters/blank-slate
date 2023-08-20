use leptos::{*, tracing::instrument};
use leptos_meta::*;
use leptos_router::*;
use log::{info, debug, warn, error};

pub mod error_template;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    info!("Rendering Home page");

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blank-slate.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[instrument(name = "HomePage")]
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
        info!("Counter Hit");
    };
    
    info!("Rendering in Home");
    warn!("Warning Test");
    debug!("Debug Test");
    error!("Error Test");

    view! { cx,
        <div class="grid grid-flow-col grid-cols-2 gap-4 m-4 space-x-10">
            <div class="container mx-auto bg-gray-600 rounded-xl grid grid-cols-1 gap-4 p-2">
                <h1 class="text-3xl font-bold underline my-2 mx-auto">"Welcome to Leptos!"</h1>
                <button class="bg-sky-500 hover:bg-sky-700 rounded-full my-2 mx-auto p-2" on:click=on_click>"Click Me: " {count}</button>
            </div>
            <div class="container mx-auto bg-gray-600 rounded-xl grid grid-cols-1 gap-4 p-2">
                <h1 class="text-3xl font-bold underline my-2 mx-auto">"Welcome to Leptos!"</h1>
                <button class="bg-sky-500 hover:bg-sky-700 rounded-full my-2 mx-auto p-2" on:click=on_click>"Click Me: " {count}</button>
            </div>
        </div>
    }
}
