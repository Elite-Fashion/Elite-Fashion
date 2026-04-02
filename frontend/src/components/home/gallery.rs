use leptos::*;
use crate::config::AppConfig;

#[component]
pub fn Gallery() -> impl IntoView {
    let (selected_image, set_selected_image) = create_signal(0);
    let (lightbox_open, set_lightbox_open) = create_signal(false);
    
    let config = AppConfig::new();
    
    let gallery_images = vec![
        config.static_url("/images/product-1.webp"),
        config.static_url("/images/product-2.webp"), 
        config.static_url("/images/product-3.webp"),
        config.static_url("/images/product-4.webp"),
        config.static_url("/images/product-5.webp"),
        config.static_url("/images/product-6.webp"),
    ];
    
    let gallery_images_len = gallery_images.len();
    
    let open_lightbox = move |index: usize| {
        set_selected_image.set(index);
        set_lightbox_open.set(true);
    };
    
    let close_lightbox = move |_| {
        set_lightbox_open.set(false);
    };
    
    let next_image = move |_| {
        let current = selected_image.get();
        set_selected_image.set((current + 1) % gallery_images_len);
    };
    
    let prev_image = move |_| {
        let current = selected_image.get();
        set_selected_image.set(if current == 0 { gallery_images_len - 1 } else { current - 1 });
    };
    
    view! {
        <section id="gallery" class="gallery">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title">"Product Gallery"</h2>
                    <p class="section-subtitle">"Explore our product from every angle"</p>
                </div>
                
                <div class="gallery-grid">
                    {gallery_images.iter().enumerate().map(|(index, image_src)| {
                        let img_src = image_src.clone();
                        
                        view! {
                            <div 
                                class="gallery-item"
                                on:click=move |_| open_lightbox(index)
                            >
                                <img src=img_src alt="Gallery Image" class="gallery-img"/>
                                <div class="gallery-overlay">
                                    <span class="view-icon">"🔍"</span>
                                    <span class="view-text">"View Details"</span>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
                
                <div class="gallery-controls">
                    <button class="gallery-btn" on:click=prev_image>
                        "← Previous"
                    </button>
                    <span class="gallery-counter">
                        {selected_image.get() + 1} " / " {gallery_images_len}
                    </span>
                    <button class="gallery-btn" on:click=next_image>
                        "Next →"
                    </button>
                </div>
            </div>
            
            // Lightbox
            <div 
                class="lightbox" 
                class:lightbox-open=lightbox_open
                on:click=close_lightbox
            >
                <div class="lightbox-content" on:click=|e| e.stop_propagation()>
                    <button class="lightbox-close" on:click=close_lightbox>
                        "×"
                    </button>
                    <img 
                        src={move || gallery_images[selected_image.get()].clone()} 
                        alt="Lightbox Image" 
                        class="lightbox-img"
                    />
                    <div class="lightbox-controls">
                        <button class="lightbox-nav" on:click=prev_image>
                            "←"
                        </button>
                        <button class="lightbox-nav" on:click=next_image>
                            "→"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
