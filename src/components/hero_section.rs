use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn HeroSection() -> impl IntoView {
    let (mobileMenuOpen, setMobileMenuOpen) = create_signal(false);

    view! {
        <div class="min-h-screen overflow-auto flex flex-col">
        <div class="bg-white h-full">
            <header class="absolute inset-x-0 top-0 z-50">
                <nav class="flex items-center justify-between p-6 lg:px-8" aria-label="Global">
                <div class="flex lg:flex-1">
                    <a href="#" class="-m-1.5 p-1.5">
                    <span class="sr-only">LicenciYa</span>
                    <img
                    src="assets/logo.svg"
                    />
                    </a>
                </div>
                <div class="flex lg:hidden">
                    <button 
                        type="button" 
                        class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700"
                        on:click=move |_| setMobileMenuOpen(true)
                    >
                    <span class="sr-only">LicenciYa menu</span>
                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                    </svg>
                    </button>
                </div>
                <div class="hidden lg:flex lg:gap-x-12">
                    <a href="#home" class="text-sm font-semibold leading-6 text-gray-900">Inicio</a>
                </div>
                <div class="hidden lg:flex lg:flex-1 lg:justify-end">
                    //<a href="#" class="text-sm font-semibold leading-6 text-gray-900">Log in <span aria-hidden="true">&rarr;</span></a>
                </div>
                </nav>
                {move || match mobileMenuOpen() {
                    true => {
                        println!("{}",mobileMenuOpen());
                        // returns HtmlElement<Pre>
                        view! { 
                            <div class="lg:hidden" role="dialog" aria-modal="true">
                            <div class="fixed inset-0 z-50"></div>
                            <div class="fixed inset-y-0 right-0 z-50 w-full overflow-y-auto bg-white px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10">
                                <div class="flex items-center justify-between">
                                <a href="#" class="-m-1.5 p-1.5">
                                    <span class="sr-only">LicenciYa</span>
                                    <img class="h-8 w-auto" src="assets/logo.svg" alt=""/>
                                </a>
                                <button 
                                    type="button" 
                                    class="-m-2.5 rounded-md p-2.5 text-gray-700"
                                    on:click=move |_| setMobileMenuOpen(false)
                                    >
                                    <span class="sr-only">Close menu</span>
                                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                                </div>
                                <div class="mt-6 flow-root">
                                <div class="-my-6 divide-y divide-gray-500/10">
                                    <div class="space-y-2 py-6">
                                    <a href="#home" class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50">Inicio</a>
                                    </div>
                                    //<div class="py-6">
                                    //<a href="#" class="-mx-3 block rounded-lg px-3 py-2.5 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50">Log in</a>
                                    //</div>
                                </div>
                                </div>
                            </div>
                            </div>
                         }
                    },
                    false => {
                        // returns HtmlElement<P>
                        view! { <div> </div> }
                    }
                    // returns HtmlElement<Textarea>
                    _ => view! { <div> </div> }
                }}
            </header>

            <div id="home" class="relative isolate px-6 lg:px-8 h-screen flex justify-center items-center">
                <div class="mx-auto max-w-2xl py-32 my-60 sm:py-48 lg:py-56">
                <div class="hidden sm:mb-8 sm:flex sm:justify-center">
                    <div class="relative rounded-full px-3 py-1 text-sm leading-6 text-licenciya-blue ring-1 ring-gray-900/10 hover:ring-gray-900/20">
                    Practica para tu examen teórico de forma entretenida
                    </div>
                </div>
                <div class="text-center">
                    <h1 class="text-3xl font-bold tracking-tight text-gray-900 md:text-5xl sm:text-6xl">"¿Listo para rendir tu examen teórico de manejo?"</h1>
                    <p class="mt-6 text-md md:text-md sm:text-lg leading-8 text-gray-600">Con LicenciYa, puedes estudiar de manera efectiva y entretenida. Nuestra aplicación te ofrece diferentes niveles de dificultad para que puedas aprender a tu propio ritmo.</p>
                    <div class="mt-10 flex items-center justify-center gap-x-6">
                    <a href="#" class="rounded-md bg-licenciya-blue px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">"¡Comienza ahora mismo!"</a>
                    </div>
                </div>
                </div>
            </div>
            </div>
            <footer class="bg-licenciya-blue text-white py-4 px-6 lg:px-8">
            <div class="container mx-auto flex justify-between items-center">
            <div>
                    <p class="text-sm font-semibold">LicenciYa</p>
                    <p class="text-xs"> 2024 Todos los derechos reservados.</p>
                </div>
                <div class="flex items-center">
                    {/* Aquí puedes agregar los íconos de tus redes sociales */}
                    <a href="#" class="text-gray-300 hover:text-white mx-2"><i class="fab fa-facebook"></i></a>
                    <a href="#" class="text-gray-300 hover:text-white mx-2"><i class="fab fa-twitter"></i></a>
                    <a href="#" class="text-gray-300 hover:text-white mx-2"><i class="fab fa-instagram"></i></a>
                </div>
            </div>
        </footer>
            </div>
    }
}
