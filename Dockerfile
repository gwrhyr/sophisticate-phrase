FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY templates ./templates
COPY .sqlx/ ./.sqlx/

ENV SQLX_OFFLINE=true

RUN cargo build --release

EXPOSE 3000

CMD ["/app/target/release/sophisticate-phrase"]
