FROM rust AS builder 

WORKDIR /app

COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release 

FROM debian:stable-slim AS runner 

COPY --from=BUILDER /app/target/release/log_monitoring /app/log_monitoring

WORKDIR /run

ENTRYPOINT ["/app/log_monitoring"]