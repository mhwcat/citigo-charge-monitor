FROM rust:1-alpine as chef
WORKDIR /app
RUN apk add --update alpine-sdk && cargo install cargo-chef

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin citigo-charge-monitor-api-server

FROM alpine:3
WORKDIR /app
COPY --from=builder /app/target/release/citigo-charge-monitor-api-server /app/log4rs.yml /app/
RUN apk add --update curl tzdata
HEALTHCHECK CMD curl --fail http://localhost:8000/api/health || exit 1     

ENTRYPOINT ["./citigo-charge-monitor-api-server"]