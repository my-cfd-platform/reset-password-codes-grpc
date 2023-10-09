use std::sync::Arc;

use app::AppContext;
use grpc::grpc_service::GrpcService;
use reset_password_codes_grpc::reset_password_codes_service_server::ResetPasswordCodesServiceServer;

mod app;
mod flows;
mod grpc;
mod settings;

#[allow(non_snake_case)]
pub mod reset_password_codes_grpc {
    tonic::include_proto!("reset_password_codes");
}

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".my-cfd-platform").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;
    let app_context = Arc::new(AppContext::new(&settings_reader).await);

    service_context.configure_grpc_server(|builder| {
        builder.add_grpc_service(ResetPasswordCodesServiceServer::new(GrpcService::new(
            app_context.clone(),
        )))
    });

    service_context.start_application().await;

    /*
    let settings_reader = crate::settings::SettingsReader::new(".my-cfd-platform").await;

    let settings_reader = Arc::new(settings_reader);

    SeqLogger::enable_from_connection_string(settings_reader.clone());

    my_logger::LOGGER
        .populate_app_and_version(crate::app::APP_NAME, crate::app::APP_VERSION)
        .await;

    let telemetry_writer = my_telemetry_writer::MyTelemetryWriter::new(
        crate::app::APP_NAME.to_string(),
        settings_reader.clone(),
    );

    let app = app::AppContext::new(&settings_reader).await;

    let app = Arc::new(app);
    http_is_alive_shared::start_up::start_server(
        app::APP_NAME.to_string(),
        app::APP_VERSION.to_string(),
        app.app_states.clone(),
    );

    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    tokio::spawn(crate::grpc::server::start(app.clone(), 8888));

    app.app_states.wait_until_shutdown().await;
     */
}
