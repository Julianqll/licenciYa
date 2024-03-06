use leptos::*;
use leptos_router::use_params_map;

use crate::{components::{simulacro_dificil::SimulacroDificil, simulacro_facil::SimulacroFacil, simulacro_medio::SimulacroMedio, simulacro_section::SimulacroSection}, pages::not_found::NotFound};

/// Default Home Page
#[component]
pub fn Simulacro() -> impl IntoView {

    let params = use_params_map();
    let type_simulacro = move || params.with(|params| params.get("type").cloned().unwrap_or_default());

    view! { 
        {move || match type_simulacro().as_str() {
            "facil" => {
                view! { <SimulacroFacil/> }
            },
            "medio" => {
                // returns HtmlElement<P>
                view! { <SimulacroMedio/> }
            }
            "dificil" => {
                // returns HtmlElement<P>
                view! { <SimulacroDificil/>}
            }
            // returns HtmlElement<Textarea>
            _ => view! { <NotFound /> }
        }}
     }

}
