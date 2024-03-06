use crate::components::header::Header;
use crate::components::hero_section::HeroSection;
use crate::components::simulacro_section::SimulacroSection;
use crate::components::footer::Footer;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen overflow-auto flex flex-col">
            <div class="bg-white h-full">
                <Header/>
                <HeroSection/>
                <SimulacroSection/>
                <Footer/>
            </div>
        </div>
    }
}
