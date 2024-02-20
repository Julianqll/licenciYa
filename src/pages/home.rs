use crate::components::hero_section::HeroSection;
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <HeroSection/>
    }
}
