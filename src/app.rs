use crate::{
    error_template::ErrorTemplate,
    routes::{
        hello_world::HelloWorld,
        imports::ImportListings,
        listings::{AddListing, Listing, Listings, ListingsLayout},
        meta::{Accounts, AccountsLayout, Me, MetaDashboard, Posts},
    },
    utilities::app_error::AppError,
};
use http::StatusCode;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/conflux.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Vec::<AppError>::new();
            outside_errors.push(AppError::new(StatusCode::NOT_FOUND, "Not found"));
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <nav>
                    <a class="nav-button" href="/">"Home"</a>
                    <a class="nav-button" href="/hello">"Hello"</a>
                    <a class="nav-button" href="/import">"Import Listings"</a>
                    <a class="nav-button" href="/listings">"Listings"</a>
                    <a class="nav-button" href="/meta">Meta</a>
                </nav>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/hello" view=HelloWorld />
                    <Route path="/import" view=ImportListings />
                    <Route path="/listings" view=ListingsLayout>
                        <Route path="" view=Listings/>
                        <Route path=":id" view=Listing/>
                        <Route path="add" view=AddListing />
                    </Route>
                    <Route path="/meta" view=MetaDashboard >
                        <Route path="" view=Me />
                        <Route path="accounts" view=AccountsLayout >
                            <Route path="" view=Accounts />
                            <Route path=":page_id" view=Posts />
                        </Route>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
