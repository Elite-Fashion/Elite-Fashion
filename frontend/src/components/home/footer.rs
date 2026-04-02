use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer id="contact" class="footer">
            <div class="container">
                <div class="footer-content">
                    <div class="footer-section">
                        <h3 class="footer-title">"Elite Fashion"</h3>
                        <p class="footer-description">
                            "Transforming the future with innovative Fashion that empower individuals worldwide."
                        </p>
                        <div class="social-links">
                            <a href="#" class="social-link">"📧"</a>
                            <a href="#" class="social-link">"💼"</a>
                            <a href="#" class="social-link">"🐦"</a>
                            <a href="#" class="social-link">"📷"</a>
                        </div>
                    </div>
                    
                    <div class="footer-section">
                        <h4 class="footer-heading">"Product"</h4>
                        <ul class="footer-links">
                            <li><a href="#features">"Features"</a></li>
                            <li><a href="#gallery">"Gallery"</a></li>
                            <li><a href="#testimonials">"Reviews"</a></li>
                            <li><a href="#Hero">"Pricing"</a></li>
                        </ul>
                    </div>
                    
                    <div class="footer-section">
                        <h4 class="footer-heading">"Company"</h4>
                        <ul class="footer-links">
                            <li><a href="#">"About Us"</a></li>
                            <li><a href="#">"Careers"</a></li>
                            <li><a href="#">"Blog"</a></li>
                            <li><a href="#">"Press"</a></li>
                        </ul>
                    </div>
                    
                    <div class="footer-section">
                        <h4 class="footer-heading">"Support"</h4>
                        <ul class="footer-links">
                            <li><a href="#">"Documentation"</a></li>
                            <li><a href="#">"Help Center"</a></li>
                            <li><a href="#">"Contact Us"</a></li>
                            <li><a href="#">"Status"</a></li>
                        </ul>
                    </div>
                </div>
                
                <div class="footer-bottom">
                    <div class="footer-brand">
                        <h3>"Elite Fashion"</h3>
                        <p>"Discover timeless elegance and modern style in our exclusive clothing collection"</p>
                    </div>
                    <div class="footer-copyright">
                        <p>"© 2026 Elite Fashion. All rights reserved."</p>
                    </div>
                    <div class="footer-legal">
                        <a href="#">"Privacy Policy"</a>
                        <a href="#">"Terms of Service"</a>
                        <a href="#">"Cookie Policy"</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
pub fn CallToAction() -> impl IntoView {
    view! {
        <section id="contact" class="cta">
            <div class="container">
                <div class="cta-content">
                    <h2 class="cta-title">"Ready to Elevate Your Style?"</h2>
                    <p class="cta-subtitle">"Discover our exclusive collection and find pieces that define your unique fashion statement"</p>
                    <div class="cta-buttons">
                        <button class="btn btn-primary btn-large">
                            "Shop Collection"
                        </button>
                        <button class="btn btn-secondary btn-large">
                            "Style Consultation"
                        </button>
                    </div>
                    <div class="cta-features">
                        <div class="cta-feature">
                            <span class="feature-icon">"🚚"</span>
                            <span>"Free Shipping Worldwide"</span>
                        </div>
                        <div class="cta-feature">
                            <span class="feature-icon">"🔄"</span>
                            <span>"30-Day Easy Returns"</span>
                        </div>
                        <div class="cta-feature">
                            <span class="feature-icon">"💎"</span>
                            <span>"Premium Quality Guaranteed"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
