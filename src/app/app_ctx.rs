use std::sync::Arc;

use service_sdk::rust_extensions::AppStates;

use super::GeneratedCodesRepo;

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub codes: GeneratedCodesRepo,
    pub settings_reader: Arc<crate::settings::SettingsReader>,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            codes: GeneratedCodesRepo::new(),
            settings_reader: settings_reader.clone(),
        }
    }
}
