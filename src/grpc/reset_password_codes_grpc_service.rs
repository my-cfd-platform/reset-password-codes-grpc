use super::grpc_service::GrpcService;
use crate::reset_password_codes_grpc::reset_password_codes_service_server::ResetPasswordCodesService;
use crate::reset_password_codes_grpc::*;

service_sdk::macros::use_grpc_server!();

#[tonic::async_trait]
impl ResetPasswordCodesService for GrpcService {
    #[with_telemetry]
    async fn generate_reset_password_code(
        &self,
        request: tonic::Request<GenerateResetPasswordCodeRequest>,
    ) -> Result<tonic::Response<GenerateResetPasswordCodeResponse>, tonic::Status> {
        let request = request.into_inner();

        let code =
            crate::flows::generate_reset_password_code(&self.app, request.trader_id, request.email)
                .await;

        return Ok(tonic::Response::new(GenerateResetPasswordCodeResponse {
            code,
        }));
    }
    #[with_telemetry]
    async fn verify_code(
        &self,
        request: tonic::Request<VerifyCodeRequest>,
    ) -> Result<tonic::Response<VerifyCodeResponse>, tonic::Status> {
        let request = request.into_inner();

        let trader_id =
            crate::flows::get_trader_id_by_code(&self.app, request.email, request.code).await;

        match trader_id {
            Some(trader_id) => Ok(tonic::Response::new(VerifyCodeResponse { trader_id })),
            None => Ok(tonic::Response::new(VerifyCodeResponse {
                trader_id: "".into(),
            })),
        }
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
