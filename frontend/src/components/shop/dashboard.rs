use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <main style="padding: 120px 15px; color: white;">
            <div style="max-width: 1200px; margin: 0 auto;">
                <h1 style="font-size: 2.2rem; margin-bottom: 25px;">"Dashboard"</h1>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px;">
                    <div style="background: rgba(102, 126, 234, 0.2); padding: 25px; border-radius: 10px; border: 1px solid rgba(102, 126, 234, 0.3);">
                        <h3 style="color: #667eea; margin-bottom: 10px;">"Statistics"</h3>
                        <p style="opacity: 0.8;">"View your shop statistics and analytics"</p>
                    </div>
                    <div style="background: rgba(102, 126, 234, 0.2); padding: 25px; border-radius: 10px; border: 1px solid rgba(102, 126, 234, 0.3);">
                        <h3 style="color: #667eea; margin-bottom: 10px;">"Settings"</h3>
                        <p style="opacity: 0.8;">"Configure your shop preferences"</p>
                    </div>
                    <div style="background: rgba(102, 126, 234, 0.2); padding: 25px; border-radius: 10px; border: 1px solid rgba(102, 126, 234, 0.3);">
                        <h3 style="color: #667eea; margin-bottom: 10px;">"Reports"</h3>
                        <p style="opacity: 0.8;">"Generate and view reports"</p>
                    </div>
                </div>
            </div>
        </main>
    }
}
