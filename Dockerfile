FROM rust:slim
COPY ./target/release/reset-password-codes-grpc ./target/reset-password-codes-grpc
ENTRYPOINT ["./target/release/reset-password-codes-grpc"]