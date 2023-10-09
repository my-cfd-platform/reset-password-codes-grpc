FROM ubuntu:22.04
COPY ./target/release/reset-password-codes-grpc ./target/reset-password-codes-grpc
ENTRYPOINT ["./target/reset-password-codes-grpc"]