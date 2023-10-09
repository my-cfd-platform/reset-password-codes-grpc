use std::sync::Arc;

use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app::AppContext;

pub async fn get_trader_id_by_code(
    app: &Arc<AppContext>,
    email: String,
    code: String,
) -> Option<String> {
    let now = DateTimeAsMicroseconds::now();
    let found_code = app.codes.find_by_email(email.as_str(), now).await?;

    if found_code.code != code {
        return None;
    }

    Some(found_code.trader_id.clone())
}
