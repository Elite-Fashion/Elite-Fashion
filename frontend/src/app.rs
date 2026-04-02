use leptos::*;
use leptos_router::*;
use crate::components::{home::app::HomeApp, shop::{app::ShopApp, app::DashboardApp, app::ProfileApp}};
use crate::config::AppConfig;

#[component]
pub fn App() -> impl IntoView {
    let config = AppConfig::new();
    
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
