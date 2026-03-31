use leptos::*;

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <section id="features" class="features" style="background: linear-gradient(135deg, #be9471 0%, #E0F6FF 100%); padding: 80px 0;">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title" style="color: #333; font-size: 2.5rem; margin-bottom: 10px; font-weight: bold;">"Amazing Features"</h2>
                    <p class="section-subtitle" style="color: #666; font-size: 1.2rem;">"Discover what makes our product incredible"</p>
                </div>
                
                <div class="features-grid">
                    {features_data().into_iter().enumerate().map(move |(_index, feature)| {
                        view! {
                            <div class="feature-card feature-visible" 
                                 style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); 
                                        border: none; box-shadow: 0 10px 30px rgba(102, 126, 234, 0.3);
                                        transition: all 0.3s ease; cursor: pointer; border-radius: 15px; padding: 30px;"
                                 on:mouseenter=move |_| {
                                     // Add hover effect
                                 }
                                 on:mouseleave=move |_| {
                                     // Remove hover effect
                                 }>
                                <div class="feature-icon" style="background: rgba(255, 255, 255, 0.2); border-radius: 50%; width: 80px; height: 80px; display: flex; align-items: center; justify-content: center; margin-bottom: 20px;">
                                    <span class="icon-emoji" style="font-size: 2.5rem;">{feature.icon}</span>
                                </div>
                                <h3 class="feature-title" style="color: white; font-size: 1.5rem; margin-bottom: 15px; font-weight: bold;">{feature.title}</h3>
                                <p class="feature-description" style="color: rgba(255, 255, 255, 0.9); line-height: 1.6;">{feature.description}</p>
                                <div class="feature-stats" style="margin-top: 20px;">
                                    <div class="stat" style="background: rgba(255, 255, 255, 0.2); padding: 12px 20px; border-radius: 25px;">
                                        <span class="stat-number" style="color: white; font-weight: bold; font-size: 1.2rem;">{feature.stat_number}</span>
                                        <span class="stat-label" style="color: rgba(255, 255, 255, 0.9); margin-left: 8px; font-size: 0.9rem;">{feature.stat_label}</span>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

fn features_data() -> Vec<Feature> {
    vec![
        Feature {
            icon: "👔",
            title: "Premium Quality",
            description: "Expertly crafted from the finest materials for exceptional comfort and durability that lasts.",
            stat_number: "100%",
            stat_label: "Quality",
        },
        Feature {
            icon: "🎨",
            title: "Designer Styles",
            description: "Curated collections from renowned designers featuring the latest fashion trends and timeless classics.",
            stat_number: "50+",
            stat_label: "Designs",
        },
        Feature {
            icon: "🌍",
            title: "Sustainable Fashion",
            description: "Eco-friendly materials and ethical production practices that care for both style and our planet.",
            stat_number: "Eco",
            stat_label: "Certified",
        },
        Feature {
            icon: "✨",
            title: "Perfect Fit",
            description: "Tailored to perfection with attention to detail ensuring every piece fits like it was made for you.",
            stat_number: "Perfect",
            stat_label: "Fit",
        },
        Feature {
            icon: "🚀",
            title: "Fast Delivery",
            description: "Quick and reliable shipping worldwide so you can enjoy your new fashion pieces without the wait.",
            stat_number: "2-3",
            stat_label: "Days",
        },
        Feature {
            icon: "💎",
            title: "Exclusive Collection",
            description: "Limited edition pieces and exclusive designs you won't find anywhere else for unique style.",
            stat_number: "Limited",
            stat_label: "Edition",
        },
    ]
}

struct Feature {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    stat_number: &'static str,
    stat_label: &'static str,
}
