use leptos::*;
use web_sys::MouseEvent;

use crate::{api::preguntas::preguntas, resources::question::Question};
/// A parameterized incrementing button
#[component]
pub fn SimulacroFacil() -> impl IntoView {
    let load_questions= create_resource(|| (), |_| async move {
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
    
    let questions = {
        move || match load_questions.get() {
            None => Vec::new(), // Vector vacío si no hay datos
            Some(json_data) => {
                // Manejar el error aquí
                serde_json::from_str::<Vec<Question>>(json_data.unwrap().as_ref())
                    .unwrap_or_else(|e| {
                        // Manejar el error de deserialización aquí
                        eprintln!("Error al deserializar las preguntas: {}", e);
                        Vec::new() // Vector vacío en caso de error
                    })
            }
        }
    };
    
    let (q_index, set_q_index) = create_signal(0);
    let (form_state, set_form_state) = create_signal(0);
    let (points, set_points) = create_signal(0);

    let next_form_state = move |_ :MouseEvent| set_form_state.update(|form_state| *form_state += 1);

    let next_q_index = move |is_correct :bool| move |_ :MouseEvent| {
        set_q_index.update(|q_index| *q_index += 1);
        if is_correct.to_owned()
        {
            set_points.update(|points| *points += 1);
        }
    };

    create_effect(move |_| {
        if q_index.get() == 3
        {
            set_form_state.update(|form_state| *form_state += 1);
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
                    <div class="text-center">
                        <h1 class="text-3xl font-bold tracking-tight mb-5 text-gray-900 md:text-5xl sm:text-6xl">"40:00:00"</h1>
                        {
                            move || match form_state.get() {
                                0 => view! {<p>"Aquí iran apareciendo las preguntas y debajo las preguntas. Para iniciar el examen solo da clic a Iniciar"</p>}.into_view(),
                                1 => {
                                    let question = &questions()[q_index.get()];
                                    view! {<p>{&question.title}</p>}.into_view()
                                }, 
                                2 => view! {<p>"Felicitaciones, terminaste el simulacro. Este es tu puntaje:" {move || points.get()}</p>}.into_view(),
                                _ => view! {<p></p>}.into_view()
                            }
                        
                        }                        
                        <div class="mt-10 grid grid-cols-1 gap-y-6">
                        {
                            move || match form_state.get() {
                                0 => view! {
                                    <button on:click=next_form_state class="flex items-center justify-center rounded-md border border-transparent bg-licenciya-blue px-8 py-3 text-base font-medium text-white hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">"¡Iniciar!"</button>
                                }.into_view(),
                                1 => {
                                    let question = &questions()[q_index.get()];
                                    let view_answers  = question.answerList.clone().into_iter()
                                    .map(|answer| view! 
                                        {
                                            <button on:click=next_q_index({answer.isCorrect}) class="flex items-center justify-center rounded-md border border-transparent bg-licenciya-blue  px-8 py-3 text-base font-medium text-white hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">{answer.body}</button>
                                        })
                                        .collect::<Vec<_>>();
    
                                    view! { {view_answers}  }.into_view()
                                }, 
                                2 => view! {
                                    <a href="/#simulacro" class="rounded-md bg-licenciya-blue px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">"Regresar al inicio"</a>
                                }.into_view(),
                                _ => view! {<p></p>}.into_view()
                            }
                        
                        } 
                        </div>
                    </div>
                    </div>
                </div>
                //fin de sección    
            </div>
        </div>

    }
}
