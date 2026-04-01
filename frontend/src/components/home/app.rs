use leptos::*;
use crate::components::home::{hero::Hero, features::Features, gallery::Gallery, testimonials::Testimonials, footer::Footer, navigation::Navigation};

#[component]
pub fn HomeApp() -> impl IntoView {
    view! {
        <div class="homeapp">
            <Navigation/>
            <div class="content">
                <Hero/>
                <Features/>
                <Gallery/>
                <Testimonials/>
                <Footer/>
            </div>
        </div>
    }
}
