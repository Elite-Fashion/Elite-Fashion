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
        <nav class:scrolled=scrolled style="
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 1000;
            background-color: rgba(26, 26, 46, 0.8);
            backdrop-filter: blur(10px);
            transition: all 0.3s ease;
            padding: 15px 0;
            border-bottom: 1px solid rgba(102, 126, 234, 0.2);
        " style:background-color=move || if scrolled.get() { "rgba(26, 26, 46, 0.95)" } else { "rgba(26, 26, 46, 0.8)" }>
            <div style="max-width: 1200px; margin: 0 auto; padding: 0 20px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap;">
                <div style="color: white; font-size: 1.5rem; font-weight: bold; margin-bottom: 0;">
                    "Shop Dashboard"
                </div>
                <div style="display: flex; gap: 20px; align-items: center; flex-wrap: wrap;">
                    <A href={config.route_url("/shop")} class="nav-link2">
                        "Shop"
                    </A>
                    <A href={config.route_url("/shop/dashboard")} class="nav-link2">
                        "Dashboard"
                    </A>
                    <A href={config.route_url("/shop/profile")} class="nav-link2">
                        "Profile"
                    </A>
                    <A href={config.route_url("/")} class="back-link">
                        "← Home"
                    </A>
                </div>
            </div>
        </nav>
    }
}
