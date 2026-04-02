pub struct AppConfig {
    pub base_url: String,
}

impl AppConfig {
    pub fn new() -> Self {
        let base_url = if cfg!(feature = "production") {
            "/Elite-Fashion"
        } else {
            ""
        };
        
        Self { base_url: base_url.to_string() }
    }
    
    pub fn static_url(&self, path: &str) -> String {
        format!("{}/static{}", self.base_url, path)
    }
    
    pub fn route_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
