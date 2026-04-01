use leptos::*;

#[component]
pub fn Shop() -> impl IntoView {
    view! {
        <div class="shop-page" style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 100px 20px 40px; overflow-y: auto;">
            <div class="container" style="max-width: 1200px; margin: 0 auto;">
                <div class="shop-header" style="text-align: center; margin-bottom: 60px;">
                    <h1 style="color: white; font-size: 3rem; margin-bottom: 20px;">"Elite Fashion Shop"</h1>
                    <p style="color: rgba(255, 255, 255, 0.9); font-size: 1.2rem;">"Discover our premium collection"</p>
                </div>
                
                <div class="shop-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 30px; margin-bottom: 60px;">
                    {shop_items().into_iter().map(|item| {
                        view! {
                            <div class="shop-item" style="background: white; border-radius: 15px; padding: 30px; text-align: center; box-shadow: 0 10px 30px rgba(0,0,0,0.1); transition: all 0.3s ease; cursor: pointer;"
                                 on:mouseenter=move |_| {}
                                 on:mouseleave=move |_| {}>
                                <div class="item-image" style="width: 200px; height: 200px; background: linear-gradient(135deg, #f0f0f0 0%, #e0e0e0 100%); border-radius: 10px; margin: 0 auto 20px; display: flex; align-items: center; justify-content: center;">
                                    <span style="font-size: 3rem;">{item.icon}</span>
                                </div>
                                <h3 style="color: #333; font-size: 1.5rem; margin-bottom: 10px;">{item.name}</h3>
                                <p style="color: #666; line-height: 1.6; margin-bottom: 20px;">{item.description}</p>
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <span style="color: #667eea; font-size: 1.5rem; font-weight: bold;">{item.price}</span>
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
                    <a href="/Elite-Fashion/" style="background: white; color: #667eea; text-decoration: none; padding: 15px 30px; border-radius: 30px; font-weight: 500; transition: all 0.3s ease; display: inline-block;">
                        "← Back to Home"
                    </a>
                </div>
            </div>
        </div>
    }
}

struct ShopItem {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
    price: &'static str,
}

fn shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem {
            icon: "👔",
            name: "Premium Business Suit",
            description: "Expertly tailored for the modern professional. Premium fabric with exceptional comfort.",
            price: "$299",
        },
        ShopItem {
            icon: "👗",
            name: "Elegant Evening Dress",
            description: "Perfect for special occasions. Flowing design with luxurious materials.",
            price: "$249",
        },
        ShopItem {
            icon: "👕",
            name: "Designer Casual Shirt",
            description: "Versatile style for any setting. Premium cotton blend for all-day comfort.",
            price: "$149",
        },
        ShopItem {
            icon: "👚",
            name: "Fashion Trousers",
            description: "Modern cut with classic appeal. Perfect fit for every body type.",
            price: "$179",
        },
        ShopItem {
            icon: "🧥",
            name: "Luxury Coat",
            description: "Timeless elegance meets modern design. Premium wool blend construction.",
            price: "$399",
        },
        ShopItem {
            icon: "👟",
            name: "Designer Accessories",
            description: "Complete your look with our curated selection of premium accessories.",
            price: "$89",
        },
    ]
}
