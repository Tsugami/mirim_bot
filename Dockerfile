# FROM rust:1.62 as builder
# WORKDIR /app
# COPY . .
# RUN cargo build --release

# FROM debian:bullseye-slim as runtime
# COPY --from=builder /app/target/release/mirim /mirim

# ENTRYPOINT ["/mirim"]

FROM rust:slim-buster as builder
WORKDIR /code

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/mirim mirim

CMD ["/app/mirim"]