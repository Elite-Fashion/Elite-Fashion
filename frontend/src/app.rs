use leptos::*;
use leptos_router::*;
use crate::components::{home::app::HomeApp, shop::{app::ShopApp, app::DashboardApp, app::ProfileApp}};
use crate::config::AppConfig;

#[component]
pub fn App() -> impl IntoView {
    let config = AppConfig::new();
    
    // Use the config base_path for routing
    let base_path = config.base_url.clone();
    let base_path_static = base_path.leak();
 
    view! {
        <Router base={base_path_static}>
            <Routes>
                // Shop app routes
                <Route path="/shop" view=ShopApp/>
                <Route path="/shop/dashboard" view=DashboardApp/>
                <Route path="/shop/profile" view=ProfileApp/>
                
                // Home app routes
                <Route path="/" view=HomeApp/>
                
                // Fallback - redirect to home for unknown routes
                <Route path="/*any" view=HomeApp/>
            </Routes>
        </Router>
    }
}
