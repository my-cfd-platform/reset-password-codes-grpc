use std::sync::Arc;

use rand::Rng;
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app::{AppContext, GeneratedCode};

pub async fn generate_reset_password_code(
    app: &Arc<AppContext>,
    trader_id: String,
    email: String,
) -> String {
    let mut now = DateTimeAsMicroseconds::now();
    let code = app.codes.find_by_email(email.as_str(), now).await;

    if let Some(code) = code {
        return code.code.clone();
    }

    let code = format_to_code_string(generate_code());

    now.add_minutes(10);

    app.codes
        .add(GeneratedCode {
            email,
            code: code.clone(),
            expires: now,
            trader_id,
        })
        .await;

    code
}

fn generate_code() -> i32 {
    let mut rng = rand::thread_rng();
    let rnd: f64 = rng.gen();

    (rnd * 1000000.0) as i32
}

fn format_to_code_string(src: i32) -> String {
    let mut result = src.to_string();

    while result.len() < 6 {
        result.insert(0, '0');
    }

    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        for _ in 0..10000 {
            let code = super::generate_code();
            assert!(code >= 0 && code < 1000000);
        }
    }
}
