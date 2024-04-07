FROM rust:1.76.0-bookworm AS chef
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
ENV RUSTFLAGS --cfg=web_sys_unstable_apis
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin backend 

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev ca-certificates

EXPOSE 587
COPY --from=builder /app/target/release/backend /usr/local/bin
CMD ["/usr/local/bin/backend"]
