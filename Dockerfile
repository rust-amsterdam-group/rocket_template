ARG APP_NAME=rocket_cloud

FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ARG APP_NAME
ENV APP_NAME $APP_NAME
RUN cargo build --release --bin $APP_NAME

FROM debian:buster-slim AS runtime
WORKDIR app
ARG APP_NAME
ENV APP_NAME $APP_NAME
COPY --from=builder /app/target/release/$APP_NAME $APP_NAME
ENV APP_ENVIRONMENT production
ENV ROCKET_PORT $PORT
ENTRYPOINT ["/bin/sh", "-c", "echo ${APP_NAME}"]
