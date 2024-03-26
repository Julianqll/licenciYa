use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;
mod api;
mod resources;
mod env;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::simulacro::Simulacro;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/style/output.css"/>
        <Router>
                <Routes>
                    <Route path=format!("{}/", env::APP_PUBLIC_URL) view=Home />
                    <Route path=format!("{}/:type", env::APP_PUBLIC_URL) view=|| view! {<Simulacro/>} />
                    <Route path=format!("{}/*any", env::APP_PUBLIC_URL) view=NotFound />
                </Routes>
        </Router>

    }
}
