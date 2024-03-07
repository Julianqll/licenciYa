use leptos::*;
use wasm_bindgen::JsValue;

use crate::api::preguntas::preguntas;
/// A parameterized incrementing button
#[component]
pub fn SimulacroFacil() -> impl IntoView {
    let once= create_resource(|| (), |_| async move {
        // Llama a tu función preguntas para cargar los datos
        let repo = "Questions".to_string(); // Inserta el nombre de tu repositorio aquí
        let result = preguntas(repo).await;

        match result {
            Ok(data) => Some(data),
            Err(err) => {
                web_sys::console::error_1(&err);
                None
            }
        }
    });
    view! {
        <div class="min-h-screen overflow-auto flex flex-col">
            <div class="bg-white h-full">
                //sección
                <div id="home" class="relative isolate px-6 lg:px-8 h-screen flex justify-center items-center">
                    <div class="mx-auto max-w-2xl py-32 my-60 sm:py-48 lg:py-24">
                    <div class="hidden sm:mb-8 sm:flex sm:justify-center">
                        <div class="relative rounded-full px-3 py-1 text-sm leading-6 text-licenciya-blue ring-1 ring-gray-900/10 hover:ring-gray-900/20">
                        Nivel Fácil
                        </div>
                    </div>
                    {move || match once.get() {
                        None => view! { <p>"Loading..."</p> }.into_view(),
                        Some(data) =>  view! { <div>{ data }</div> }.into_view()                   
                    }}
                    <div class="text-center">
                        <h1 class="text-3xl font-bold tracking-tight text-gray-900 md:text-5xl sm:text-6xl">"40:00:00"</h1>
                        <p class="mt-6 text-md md:text-md sm:text-lg leading-8 text-gray-600">Este es el texto de la pregunta, pregunta pregunta pregunta.</p>
                        <div class="mt-10 grid grid-cols-1 gap-y-6">
                            <button class="flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Respuesta uno</button>
                            <button class="flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Respuesta dos</button>
                            <button class="flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Respuesta tres</button>
                            <button class="flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">Respuesta cuatro</button>
                        </div>
                    </div>
                    </div>
                </div>
                //fin de sección    
            </div>
        </div>

    }
}
