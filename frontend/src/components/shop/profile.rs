use leptos::*;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <main style="padding: 120px 15px; color: white;">
            <div style="max-width: 800px; margin: 0 auto;">
                <h1 style="font-size: 2.2rem; margin-bottom: 25px;">"Profile"</h1>
                <div style="background: rgba(255,255,255,0.1); padding: 40px; border-radius: 15px; backdrop-filter: blur(10px);">
                    <div style="display: flex; align-items: center; margin-bottom: 30px;">
                        <div style="width: 80px; height: 80px; background: linear-gradient(135deg, #667eea, #764ba2); border-radius: 50%; margin-right: 20px; display: flex; align-items: center; justify-content: center;">
                            <span style="font-size: 2rem;">"👤"</span>
                        </div>
                        <div>
                            <h2 style="margin-bottom: 5px;">"Ala L"</h2>
                            <p style="opacity: 0.8;">"Ala-L@Elite-Fashion.com"</p>
                        </div>
                    </div>
                    <div style="display: grid; gap: 20px;">
                        <div style="padding: 20px; background: rgba(255,255,255,0.05); border-radius: 10px;">
                            <h3 style="color: #667eea; margin-bottom: 10px;">"Account Settings"</h3>
                            <p style="opacity: 0.8;">"Manage your account preferences and security settings"</p>
                        </div>
                        <div style="padding: 20px; background: rgba(255,255,255,0.05); border-radius: 10px;">
                            <h3 style="color: #667eea; margin-bottom: 10px;">"Activity Log"</h3>
                            <p style="opacity: 0.8;">"View your recent activity and login history"</p>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
