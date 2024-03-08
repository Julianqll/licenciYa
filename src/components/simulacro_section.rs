use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn SimulacroSection() -> impl IntoView {

    let levels = vec![
        (
            "Fácil",
            "Si te equivocas no hay problema, te mostraremos la respuesta correcta",
            "bg-[#4CAF50]",
            "facil"
        ),
        (
            "Medio",
            "Podrás tener 5 checkpoints durante el examen, pero solo podrás equivocarte 5 veces",
            "bg-[#ebba27]",
            "medio"
        ),
        (
            "Difícil",
            "Si te equivocas, vuelvas a empezar el examen con el tiempo restante",
            "bg-[#F44336]",
            "dificil"
        )
    ];

    view! {
        <div id="simulacro" class="flex flex-col items-center justify-center mx-auto max-w-2xl text-center py-24 px-6">
            <h2 class="text-base font-semibold leading-7 text-licenciya-blue">Elija la dificultad</h2>
            <p class="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
              Es hora de dar el simulacro
            </p>
            <p class="mt-6 text-lg leading-8 text-gray-600">
              "Tendrás 40 minutos para realizar el simulacro, tiempo que se acomodará de acuerdo a la dificultad que decidas elegir. ¡Suerte!"
            </p>
          <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-4xl">
            <dl class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-10 lg:max-w-none lg:grid-cols-3 lg:gap-y-16">
            {levels.into_iter()
                .map(|(name, description, color, param)| view! 
                    {
                        <div key={name} class="relative py-10">
                        <a href=format!("/simulacro/{}", param) class=format!("rounded-md px-3.5 py-2.5 my-5 text-sm font-semibold text-white shadow-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 {}", color)>
                            {name}
                        </a>
                        <dd class="mt-2 text-base leading-7 text-gray-600">{description}</dd>
                        </div>
                    })
                    .collect::<Vec<_>>()}
            </dl>
          </div>
        </div>
    }
}
