use leptos::*;
use leptos_router::*;
use crate::config::AppConfig;

#[component]
pub fn Shop() -> impl IntoView {
    let config = AppConfig::new();
    let (selected_item, set_selected_item) = create_signal(None::<ShopItem>);
    let (lightbox_open, set_lightbox_open) = create_signal(false);
    
    let open_lightbox = move |item: ShopItem| {
        set_selected_item.set(Some(item));
        set_lightbox_open.set(true);
    };
    
    let close_lightbox = move |_| {
        set_lightbox_open.set(false);
        set_selected_item.set(None);
    };
    view! {
        <div class="shop-page" style="min-height: 100vh; background: linear-gradient(135deg, #FF6B6B 0%, #4ECDC4 50%, #45B7D1 100%); padding: 120px 15px 40px; overflow-y: auto;">
            <div class="container" style="max-width: 1200px; margin: 0 auto;">
                <div class="shop-header" style="text-align: center; margin-bottom: 40px;">
                    <h1 style="color: white; font-size: 2.2rem; margin-bottom: 15px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">"Elite Fashion Shop"</h1>
                    <p style="color: rgba(255, 255, 255, 0.95); font-size: 1.1rem; text-shadow: 1px 1px 2px rgba(0,0,0,0.2);">"Discover our premium collection"</p>
                </div>
                
                <div class="shop-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 20px; margin-bottom: 40px;">
                    {shop_items().into_iter().map(|item| {
                        let item_clone = item.clone();
                        view! {
                            <div class="shop-item" style="background: white; border-radius: 12px; padding: 20px; text-align: center; box-shadow: 0 8px 20px rgba(0,0,0,0.15); transition: all 0.3s ease; cursor: pointer;"
                                 on:mouseenter=move |_| {}
                                 on:mouseleave=move |_| {}>
                                <div class="item-image" style="width: 240px; height: 240px; background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); border-radius: 12px; margin: 0 auto 15px; display: flex; align-items: center; justify-content: center; overflow: hidden; cursor: pointer; box-shadow: 0 6px 15px rgba(0,0,0,0.1);"
                                     on:click=move |_| open_lightbox(item.clone())>
                                    <img src={item_clone.image} alt={item_clone.name} style="width: 100%; height: 100%; object-fit: cover; transition: transform 0.3s ease; border-radius: 10px;"/>
                                </div>
                                <h3 style="color: #333; font-size: 1.3rem; margin-bottom: 10px; font-weight: 600;">{item_clone.name}</h3>
                                <p style="color: #666; line-height: 1.5; margin-bottom: 15px; font-size: 0.9rem;">{item_clone.description}</p>
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 15px; flex-wrap: wrap; gap: 10px;">
                                    <span style="color: #FF6B6B; font-size: 1.3rem; font-weight: bold;">{item_clone.price}</span>
                                    <button style="background: linear-gradient(135deg, #FF6B6B 0%, #4ECDC4 100%); color: white; border: none; padding: 10px 20px; border-radius: 20px; cursor: pointer; font-weight: 600; transition: all 0.3s ease; box-shadow: 0 3px 10px rgba(255, 107, 107, 0.3); font-size: 0.9rem;"
                                            on:click=move |_| {}>
                                        "Add to Cart"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
                
                <div class="back-to-home" style="text-align: center; margin-top: 40px;">
                    <A href={config.route_url("/")} class="back-to-home-btn">
                        "← Back to Home"
                    </A>
                </div>
            </div>
        </div>
        
        // Lightbox Modal
        <div 
            class="shop-lightbox" 
            class:lightbox-open=lightbox_open
            on:click=close_lightbox
        >
            <div class="shop-lightbox-content" on:click=|e| e.stop_propagation()>
                <button class="shop-lightbox-close" on:click=close_lightbox>
                    "×"
                </button>
                <div class="shop-lightbox-grid">
                    <div class="shop-lightbox-image">
                        {move || selected_item.get().map(|item| {
                            view! {
                                <img src={item.image} alt={item.name} class="shop-lightbox-img"/>
                            }
                        })}
                    </div>
                    <div class="shop-lightbox-details">
                        {move || selected_item.get().map(|item| {
                            view! {
                                <>
                                    <h3 class="shop-lightbox-title">{item.name}</h3>
                                    <p class="shop-lightbox-description">{item.description}</p>
                                    <div class="shop-lightbox-price">
                                        <span>{item.price}</span>
                                        <button class="shop-lightbox-cart-btn">
                                            "Add to Cart"
                                        </button>
                                    </div>
                                </>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct ShopItem {
    name: &'static str,
    description: &'static str,
    price: &'static str,
    image: String,
}

fn shop_items() -> Vec<ShopItem> {
    let config = AppConfig::new();
    vec![
        ShopItem {
            name: "Premium Business Suit",
            description: "Expertly tailored for the modern professional. Premium fabric with exceptional comfort.",
            price: "$298",
            image: config.static_url("/images/Premium Business Suit.webp"),
        },
        ShopItem {
            name: "Elegant Evening Dress",
            description: "Perfect for special occasions. Flowing design with luxurious materials.",
            price: "$249",
            image: config.static_url("/images/Elegant Evening Dress.webp"),
        },
        ShopItem {
            name: "Designer Casual Shirt",
            description: "Versatile style for any setting. Premium cotton blend for all-day comfort.",
            price: "$149",
            image: config.static_url("/images/Designer Casual Shirt.webp"),
        },
        ShopItem {
            name: "Fashion Trousers",
            description: "Modern cut with classic appeal. Perfect fit for every body type.",
            price: "$179",
            image: config.static_url("/images/Fashion Trousers.webp"),
        },
        ShopItem {
            name: "Luxury Coat",
            description: "Timeless elegance meets modern design. Premium wool blend construction.",
            price: "$399",
            image: config.static_url("/images/Luxury Coat.webp"),
        },
        ShopItem {
            name: "Designer Accessories",
            description: "Complete your look with our curated selection of premium accessories.",
            price: "$89",
            image: config.static_url("/images/Designer Accessories.webp"),
        },
    ]
}
