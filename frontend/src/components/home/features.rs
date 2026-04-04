use leptos::*;

#[component]
pub fn Features() -> impl IntoView {
    // Include component-specific CSS
    let style = include_str!("features.css");
    leptos::document().head().unwrap()
        .insert_adjacent_html("beforeend", &format!("<style>{}</style>", style))
        .unwrap();
    view! {
        <section id="features" class="features">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title">"Amazing Features"</h2>
                    <p class="section-subtitle">"Discover what makes our product incredible"</p>
                </div>
                
                <div class="features-grid">
                    {features_data().into_iter().enumerate().map(move |(_index, feature)| {
                        view! {
                            <div class="feature-card feature-visible"
                                 on:mouseenter=move |_| {
                                     // Add hover effect
                                 }
                                 on:mouseleave=move |_| {
                                     // Remove hover effect
                                 }>
                                <div class="feature-icon">
                                    <span class="icon-emoji">{feature.icon}</span>
                                </div>
                                <h3 class="feature-title">{feature.title}</h3>
                                <p class="feature-description">{feature.description}</p>
                                <div class="feature-stats">
                                    <div class="stat">
                                        <span class="stat-number">{feature.stat_number}</span>
                                        <span class="stat-label">{feature.stat_label}</span>
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
