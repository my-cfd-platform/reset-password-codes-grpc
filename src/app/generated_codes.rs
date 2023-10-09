use std::sync::Arc;

use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::RwLock;

pub struct GeneratedCode {
    pub trader_id: String,
    pub email: String,
    pub code: String,
    pub expires: DateTimeAsMicroseconds,
}

impl GeneratedCode {
    pub fn is_expired(&self, now: DateTimeAsMicroseconds) -> bool {
        self.expires.unix_microseconds < now.unix_microseconds
    }
}

pub struct GeneratedCodesRepo {
    pub codes: RwLock<Vec<Arc<GeneratedCode>>>,
}

impl GeneratedCodesRepo {
    pub fn new() -> Self {
        Self {
            codes: RwLock::new(Vec::new()),
        }
    }

    pub async fn add(&self, code: GeneratedCode) {
        let mut codes = self.codes.write().await;
        codes.push(Arc::new(code));
    }

    async fn find_by_email_int(&self, email: &str) -> Option<Arc<GeneratedCode>> {
        let codes = self.codes.read().await;
        for code in codes.iter() {
            if code.email == email {
                return Some(code.clone());
            }
        }
        None
    }

    async fn remove(&self, email: &str) {
        let mut codes = self.codes.write().await;

        codes.retain(|code| code.email != email);
    }

    pub async fn find_by_email(
        &self,
        email: &str,
        now: DateTimeAsMicroseconds,
    ) -> Option<Arc<GeneratedCode>> {
        let result = self.find_by_email_int(email).await?;

        if result.is_expired(now) {
            self.remove(email).await;
            return None;
        }

        Some(result)
    }
}
