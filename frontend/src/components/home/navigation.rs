use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use crate::config::AppConfig;

#[component]
pub fn Navigation() -> impl IntoView {
    let config = AppConfig::new();
    let (scrolled, set_scrolled) = create_signal(false);
    
    // Handle scroll effect for navigation
    leptos::create_effect(move |_| {
        let window = web_sys::window().unwrap();
        let set_scrolled = set_scrolled.clone();
        let window_clone = window.clone();
        let closure = Closure::wrap(Box::new(move || {
            let scroll_y = window_clone.scroll_y().unwrap();
            set_scrolled.set(scroll_y > 50.0);
        }) as Box<dyn Fn()>);
        
        window.set_onscroll(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    });
    
    view! {
        <nav class:scrolled=scrolled>
            <div class="nav-container">
                <div class="nav-logo">
                    <h2>"Elite Fashion"</h2>
                </div>
                <div class="nav-links">
                    <A href={config.route_url("/")} class="nav-link">"Home"</A>
                    <a href="#features" class="nav-link">"Features"</a>
                    <a href="#gallery" class="nav-link">"Gallery"</a>
                    <a href="#testimonials" class="nav-link">"Reviews"</a>
                    <a href="#contact" class="nav-link">"Contact"</a>
                    //<A href={config.route_url("/shop")} class="nav-link app2-switch">"Switch to Shop"</A>
                </div>
            </div>
        </nav>
    }
}
