FROM rust:1.86.0-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --workspace


FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get install libssl-dev -y

COPY --from=builder /app/target/release/http-server /app/http-server
CMD ["/app/http-server"]
