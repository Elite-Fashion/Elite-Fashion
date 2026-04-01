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
        <Router>
            <Routes>
                <Route path={config.route_url("/shop")} view=ShopApp/>
                <Route path={config.route_url("/shop/dashboard")} view=DashboardApp/>
                <Route path={config.route_url("/shop/profile")} view=ProfileApp/>
                <Route path={config.route_url("/")} view=HomeApp/>
                <Route path={config.route_url("/*any")} view=HomeApp/>
            </Routes>
        </Router>
    }
}
