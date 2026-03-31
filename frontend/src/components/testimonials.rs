use leptos::*;

#[derive(Clone)]
#[allow(dead_code)]
struct Testimonial {
    name: &'static str,
    role: &'static str,
    avatar: &'static str,
    content: &'static str,
    rating: usize,
}

#[component]
pub fn Testimonials() -> impl IntoView {
    let (active_testimonial, set_active_testimonial) = create_signal(0);
    
    let testimonials = vec![
        Testimonial {
            name: "Sarah Mitchell",
            role: "Fashion Designer",
            avatar: "👩",
            content: "The quality of these clothes is exceptional. Every piece feels like it was custom-tailored just for me.",
            rating: 5,
        },
        Testimonial {
            name: "James Chen",
            role: "CEO",
            avatar: "👨",
            content: "Perfect for business meetings. The suits fit perfectly and the fabric is incredibly comfortable all day long.",
            rating: 5,
        },
        Testimonial {
            name: "Emma Rodriguez",
            role: "Fashion Blogger",
            avatar: "👩",
            content: "Stylish, elegant, and sustainable. This collection represents everything modern fashion should be.",
            rating: 5,
        },
        Testimonial {
            name: "Michael Park",
            role: "Photographer",
            avatar: "👨",
            content: "The attention to detail is remarkable. These pieces photograph beautifully and look even better in person.",
            rating: 5,
        },
        Testimonial {
            name: "Lisa Thompson",
            role: "Marketing Director",
            avatar: "�",
            content: "Finally, clothes that combine style with comfort. I can wear these from office to evening events effortlessly.",
            rating: 5,
        },
    ];
    
    let testimonials_len = testimonials.len();
    
    let next_testimonial = move |_| {
        let current = active_testimonial.get();
        set_active_testimonial.set((current + 1) % testimonials_len);
    };
    
    let prev_testimonial = move |_| {
        let current = active_testimonial.get();
        set_active_testimonial.set(if current == 0 { testimonials_len - 1 } else { current - 1 });
    };
    
    view! {
        <section id="testimonials" class="testimonials">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title">"What Our Customers Say"</h2>
                    <p class="section-subtitle">"Real stories from real users"</p>
                </div>
                
                <div class="testimonials-container">
                    <div class="testimonial-slider">
                        {testimonials.iter().enumerate().map(|(index, testimonial)| {
                            let is_active = active_testimonial.get() == index;
                            
                            view! {
                                <div 
                                    class="testimonial-card"
                                    class:testimonial-active=is_active
                                >
                                    <div class="testimonial-header">
                                        <div class="testimonial-avatar">
                                            <span class="avatar-emoji">{testimonial.avatar}</span>
                                        </div>
                                        <div class="testimonial-info">
                                            <h4 class="testimonial-name">{testimonial.name}</h4>
                                            <p class="testimonial-role">{testimonial.role}</p>
                                        </div>
                                        <div class="testimonial-rating">
                                            {(0..testimonial.rating).map(|_| view! { <span>"⭐"</span> }).collect_view()}
                                        </div>
                                    </div>
                                    <blockquote class="testimonial-content">
                                        "{testimonial.content}"
                                    </blockquote>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                    
                    <div class="testimonial-controls">
                        <button class="testimonial-btn" on:click=prev_testimonial>
                            "←"
                        </button>
                        <div class="testimonial-dots">
                            {(0..testimonials_len).map(|index| {
                                let is_active = active_testimonial.get() == index;
                                view! {
                                    <button 
                                        class="testimonial-dot"
                                        class:dot-active=is_active
                                        on:click=move |_| set_active_testimonial.set(index)
                                    ></button>
                                }
                            }).collect_view()}
                        </div>
                        <button class="testimonial-btn" on:click=next_testimonial>
                            "→"
                        </button>
                    </div>
                </div>
                
                <div class="testimonials-stats">
                    <div class="stat-card">
                        <h3 class="stat-number">"50K+"</h3>
                        <p class="stat-label">"Happy Customers"</p>
                    </div>
                    <div class="stat-card">
                        <h3 class="stat-number">"4.9/5"</h3>
                        <p class="stat-label">"Average Rating"</p>
                    </div>
                    <div class="stat-card">
                        <h3 class="stat-number">"98%"</h3>
                        <p class="stat-label">"Would Recommend"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
