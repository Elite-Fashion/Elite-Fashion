use leptos::*;
use leptos_router::*;
use crate::components::{home::app::HomeApp, shop::{app::ShopApp, app::DashboardApp, app::ProfileApp}};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/Elite-Fashion/">
            <Routes>
                // Shop app routes
                <Route path="/Elite-Fashion//shop" view=ShopApp/>
                <Route path="/Elite-Fashion//shop/dashboard" view=DashboardApp/>
                <Route path="/Elite-Fashion//shop/profile" view=ProfileApp/>
                
                // Home app routes
                <Route path="/Elite-Fashion/" view=HomeApp/>
                
                // Fallback
                <Route path="/Elite-Fashion/*any" view=HomeApp/>
            </Routes>
        </Router>
    }
}

