use super::server::GrpcService;
use crate::reset_password_codes_grpc::reset_password_codes_service_server::ResetPasswordCodesService;
use crate::reset_password_codes_grpc::*;

#[tonic::async_trait]
impl ResetPasswordCodesService for GrpcService {
    async fn generate_reset_password_code(
        &self,
        request: tonic::Request<GenerateResetPasswordCodeRequest>,
    ) -> Result<tonic::Response<GenerateResetPasswordCodeResponse>, tonic::Status> {
        let _ = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "generate_reset_password_code",
        );

        let request = request.into_inner();

        let code =
            crate::flows::generate_reset_password_code(&self.app, request.trader_id, request.email)
                .await;

        return Ok(tonic::Response::new(GenerateResetPasswordCodeResponse {
            code,
        }));
    }

    async fn verify_code(
        &self,
        request: tonic::Request<VerifyCodeRequest>,
    ) -> Result<tonic::Response<VerifyCodeResponse>, tonic::Status> {
        let _ = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "resolve_email_by_id",
        );

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
