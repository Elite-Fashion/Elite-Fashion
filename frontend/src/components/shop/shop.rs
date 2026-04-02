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
        <div class="shop-page" style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 100px 20px 40px; overflow-y: auto;">
            <div class="container" style="max-width: 1200px; margin: 0 auto;">
                <div class="shop-header" style="text-align: center; margin-bottom: 60px;">
                    <h1 style="color: white; font-size: 3rem; margin-bottom: 20px;">"Elite Fashion Shop"</h1>
                    <p style="color: rgba(255, 255, 255, 0.9); font-size: 1.2rem;">"Discover our premium collection"</p>
                </div>
                
                <div class="shop-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 30px; margin-bottom: 60px;">
                    {shop_items().into_iter().map(|item| {
                        let item_clone = item.clone();
                        view! {
                            <div class="shop-item" style="background: white; border-radius: 15px; padding: 30px; text-align: center; box-shadow: 0 10px 30px rgba(0,0,0,0.1); transition: all 0.3s ease; cursor: pointer;"
                                 on:mouseenter=move |_| {}
                                 on:mouseleave=move |_| {}>
                                <div class="item-image" style="width: 250px; height: 250px; background: linear-gradient(135deg, #f0f0f0 0%, #e0e0e0 100%); border-radius: 10px; margin: 0 auto 20px; display: flex; align-items: center; justify-content: center; overflow: hidden; cursor: pointer;"
                                     on:click=move |_| open_lightbox(item.clone())>
                                    <img src={item_clone.image} alt={item_clone.name} style="width: 100%; height: 100%; object-fit: cover; transition: transform 0.3s ease;"/>
                                </div>
                                <h3 style="color: #333; font-size: 1.5rem; margin-bottom: 10px;">{item_clone.name}</h3>
                                <p style="color: #666; line-height: 1.6; margin-bottom: 20px;">{item_clone.description}</p>
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 15px;">
                                    <span style="color: #667eea; font-size: 1.5rem; font-weight: bold;">{item_clone.price}</span>
                                    <button style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 12px 24px; border-radius: 25px; cursor: pointer; font-weight: 500; transition: all 0.3s ease;"
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
            price: "$299",
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
