use leptos::*;
use leptos_router::*;
use crate::components::{home::app::HomeApp, shop::{app::ShopApp, app::DashboardApp, app::ProfileApp}};

#[component]
pub fn App() -> impl IntoView {
    // Use /Elite-Fashion/ for production, / for local development
    let base_path = if cfg!(feature = "production") {
        "/Elite-Fashion/"
    } else {
        "/"
    };
 
    view! {
        <Router base={base_path}>
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
