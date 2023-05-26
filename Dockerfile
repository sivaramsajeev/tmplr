FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/tmplr /app/tmplr
ENTRYPOINT ["/app/tmplr"]
