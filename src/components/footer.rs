use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn Footer() -> impl IntoView {

    view! {
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
    }
}
