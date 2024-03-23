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
        <Router>

        // injects info into HTML tag from application code
        <Html
            lang="en"
            dir="ltr"
            attr:data-theme="light"
        />
        <head>
            <base data-trunk-public-url/>
        </head>
        // sets the document title
        <Title text="Licenciya"/>
        <Stylesheet id="leptos" href="/style/output.css"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />


        <ErrorBoundary
            fallback=|errors| view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                        .collect_view()
                    }
                </ul>
            }
        >
                <main>
                <Routes>
                    <Route path="" view=Home />
                    <Route path=":type" view=|| view! {<Simulacro/>} />
                    <Route path="*any" view=NotFound />
                </Routes>
                </main>

        </ErrorBoundary>
        </Router>

    }
}
