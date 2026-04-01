use leptos::*;
use crate::components::shop::{navigation::Navigation, shop::Shop, dashboard::Dashboard, profile::Profile};

#[component]
pub fn ShopLayout(children: Children) -> impl IntoView {
    view! {
        <div class="shopapp" style="background: linear-gradient(45deg, #1a1a2e, #16213e); min-height: 100vh;">
            <Navigation/>
            <div class="content2" style="padding-top: 0;">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn ShopApp() -> impl IntoView {
    view! {
        <ShopLayout>
            <Shop/>
        </ShopLayout>
    }
}

#[component]
pub fn DashboardApp() -> impl IntoView {
    view! {
        <ShopLayout>
            <Dashboard/>
        </ShopLayout>
    }
}

#[component]
pub fn ProfileApp() -> impl IntoView {
    view! {
        <ShopLayout>
            <Profile/>
        </ShopLayout>
    }
}
