use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn Header() -> impl IntoView {
    let (mobileMenuOpen, setMobileMenuOpen) = create_signal(false);

    view! {
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
                <a href="#simulacro" class="text-sm font-semibold leading-6 text-gray-900">Simulacro</a>
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
                                <a href="#simulacro" class="-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50">Simulacro</a>
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
                    view! { <div> </div> }
                }
            }}
        </header>
    }
}
