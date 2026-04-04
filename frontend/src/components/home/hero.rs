use leptos::*;
use leptos_router::*;
use crate::config::AppConfig;

#[component]
pub fn Hero() -> impl IntoView {
    let config = AppConfig::new();
    let (text_visible, set_text_visible) = create_signal(false);
    
    // Include component-specific CSS
    let style = include_str!("hero.css");
    leptos::document().head().unwrap()
        .insert_adjacent_html("beforeend", &format!("<style>{}</style>", style))
        .unwrap();
    
    // Trigger animation after component mounts
    leptos::create_effect(move |_| {
        leptos::set_timeout_with_handle(
            move || set_text_visible.set(true),
            std::time::Duration::from_millis(100),
        ).unwrap();
    });
    
    view! {
        <section id="home" class="hero">
            <div class="hero-background">
                <div class="hero-overlay"></div>
            </div>
            <div class="hero-content">
                <div class:hero-text-visible=text_visible class="hero-text">
                    <h1 class="hero-title">"Premium Fashion Collection"</h1>
                    <p class="hero-subtitle">"Discover timeless elegance and modern style with our exclusive clothing line"</p>
                    <div class="hero-buttons">
                    <A href={config.route_url("/shop")} class="btn btn-primary">
                            "Shop Now"
                        </A>
                        <button class="btn btn-secondary">
                            "View Collection"
                        </button>
                    </div>
                </div>
                <div class="hero-image">
                    <img src={config.static_url("/images/product-hero.webp")} alt="Product Hero" class="hero-img"/>
                </div>
            </div>
            <div class="scroll-indicator">
                <div class="scroll-arrow"></div>
                <span>"Scroll to explore"</span>
            </div>
        </section>
    }
}
