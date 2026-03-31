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
        <section id="testimonials" class="testimonials" style="background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); padding: 80px 0;">
            <div class="container">
                <div class="section-header">
                    <h2 class="section-title" style="color: #333; font-size: 2.5rem; margin-bottom: 10px;">"What Our Customers Say"</h2>
                    <p class="section-subtitle" style="color: #666; font-size: 1.2rem;">"Real stories from real users"</p>
                </div>
                
                <div class="testimonials-container">
                    <div class="testimonial-slider" style="background: white; border-radius: 20px; box-shadow: 0 20px 40px rgba(0,0,0,0.1); padding: 40px;">
                        {testimonials.iter().enumerate().map(|(index, testimonial)| {
                            let is_active = active_testimonial.get() == index;
                            
                            view! {
                                <div 
                                    class="testimonial-card"
                                    class:testimonial-active=is_active
                                    style="background: white; border-radius: 15px; padding: 30px; 
                                           box-shadow: 0 10px 30px rgba(0,0,0,0.1); transition: all 0.3s ease;
                                           border-left: 4px solid #667eea;"
                                >
                                    <div class="testimonial-header" style="display: flex; align-items: center; margin-bottom: 20px;">
                                        <div class="testimonial-avatar" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 50%; width: 60px; height: 60px; display: flex; align-items: center; justify-content: center; margin-right: 20px;">
                                            <span class="avatar-emoji" style="font-size: 1.5rem;">{testimonial.avatar}</span>
                                        </div>
                                        <div class="testimonial-info">
                                            <h4 class="testimonial-name" style="color: #333; font-size: 1.3rem; margin-bottom: 5px;">{testimonial.name}</h4>
                                            <p class="testimonial-role" style="color: #666; margin: 0;">{testimonial.role}</p>
                                        </div>
                                        <div class="testimonial-rating" style="margin-left: auto;">
                                            {(0..testimonial.rating).map(|_| view! { 
                                                <span style="color: #ffc107; font-size: 1.2rem; margin-right: 2px;">"⭐"</span> 
                                            }).collect_view()}
                                        </div>
                                    </div>
                                    <blockquote class="testimonial-content" style="color: #555; font-style: italic; font-size: 1.1rem; line-height: 1.6; margin: 0; padding-left: 20px; border-left: 2px solid #e9ecef;">
                                        "{testimonial.content}"
                                    </blockquote>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                    
                    <div class="testimonial-controls" style="margin-top: 30px; display: flex; justify-content: center; align-items: center; gap: 20px;">
                        <button class="testimonial-btn" on:click=prev_testimonial 
                                style="background: #667eea; color: white; border: none; border-radius: 50%; width: 50px; height: 50px; cursor: pointer; font-size: 1.2rem; transition: all 0.3s ease;">
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
                                        style="width: 12px; height: 12px; border-radius: 50%; border: none; margin: 0 5px; cursor: pointer; transition: all 0.3s ease;"
                                    ></button>
                                }
                            }).collect_view()}
                        </div>
                        <button class="testimonial-btn" on:click=next_testimonial
                                style="background: #667eea; color: white; border: none; border-radius: 50%; width: 50px; height: 50px; cursor: pointer; font-size: 1.2rem; transition: all 0.3s ease;">
                            "→"
                        </button>
                    </div>
                </div>
                
                <div class="testimonials-stats" style="margin-top: 60px; display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 30px;">
                    <div class="stat-card" style="background: white; padding: 30px; border-radius: 15px; text-align: center; box-shadow: 0 10px 30px rgba(0,0,0,0.1); transition: all 0.3s ease;">
                        <h3 class="stat-number" style="color: #667eea; font-size: 2.5rem; font-weight: bold; margin-bottom: 10px;">"50K+"</h3>
                        <p class="stat-label" style="color: #666; font-size: 1.1rem; margin: 0;">"Happy Customers"</p>
                    </div>
                    <div class="stat-card" style="background: white; padding: 30px; border-radius: 15px; text-align: center; box-shadow: 0 10px 30px rgba(0,0,0,0.1); transition: all 0.3s ease;">
                        <h3 class="stat-number" style="color: #667eea; font-size: 2.5rem; font-weight: bold; margin-bottom: 10px;">"4.9/5"</h3>
                        <p class="stat-label" style="color: #666; font-size: 1.1rem; margin: 0;">"Average Rating"</p>
                    </div>
                    <div class="stat-card" style="background: white; padding: 30px; border-radius: 15px; text-align: center; box-shadow: 0 10px 30px rgba(0,0,0,0.1); transition: all 0.3s ease;">
                        <h3 class="stat-number" style="color: #667eea; font-size: 2.5rem; font-weight: bold; margin-bottom: 10px;">"98%"</h3>
                        <p class="stat-label" style="color: #666; font-size: 1.1rem; margin: 0;">"Would Recommend"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
