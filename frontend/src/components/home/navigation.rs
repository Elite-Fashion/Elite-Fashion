use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;

#[component]
pub fn Navigation() -> impl IntoView {
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
                    <a href="#home" class="logo-link">
                        <h2>"Elite Fashion"</h2>
                    </a>
                </div>
                <div class="nav-links">
                    <a href="#features" class="nav-link">"Features"</a>
                    <a href="#gallery" class="nav-link">"Gallery"</a>
                    <a href="#testimonials" class="nav-link">"Reviews"</a>
                    <a href="#contact" class="nav-link">"Contact"</a>
                </div>
            </div>
        </nav>
    }
}
