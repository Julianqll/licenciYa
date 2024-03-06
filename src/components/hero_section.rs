use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn HeroSection() -> impl IntoView {

    view! {
        <div id="home" class="relative isolate px-6 lg:px-8 h-screen flex justify-center items-center">
            <div class="mx-auto max-w-2xl py-32 my-60 sm:py-48 lg:py-24">
            <div class="hidden sm:mb-8 sm:flex sm:justify-center">
                <div class="relative rounded-full px-3 py-1 text-sm leading-6 text-licenciya-blue ring-1 ring-gray-900/10 hover:ring-gray-900/20">
                Practica para tu examen teórico de forma entretenida
                </div>
            </div>
            <div class="text-center">
                <h1 class="text-3xl font-bold tracking-tight text-gray-900 md:text-5xl sm:text-6xl">"¿Listo para rendir tu examen teórico de manejo?"</h1>
                <p class="mt-6 text-md md:text-md sm:text-lg leading-8 text-gray-600">Con LicenciYa, puedes estudiar de manera efectiva y entretenida. Nuestra aplicación te ofrece diferentes niveles de dificultad para que puedas aprender a tu propio ritmo.</p>
                <div class="mt-10 flex items-center justify-center gap-x-6">
                <a href="/simulacro" class="rounded-md bg-licenciya-blue px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">"¡Comienza ahora mismo!"</a>
                </div>
            </div>
            </div>
        </div>
    }
}
