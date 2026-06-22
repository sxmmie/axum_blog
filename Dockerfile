# Stage 1
FROM rust:1.91 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/axum_blog .
EXPOSE 7870
CMD ["./axum_blog"]
