# ---------- Builder Stage ----------
FROM rust:bookworm AS builder

WORKDIR /app

# Install OpenSSL dev headers (needed by lettre/native-tls)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Cache dependencies first (huge speed boost)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy real source
COPY src ./src
COPY src-web ./src-web

# Build real binary
RUN touch src/main.rs && cargo build --release

# ---------- Runtime Stage ----------
FROM debian:bookworm-slim AS runner

# Install minimal runtime deps
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/choredom-website /app/choredom-website
COPY --from=builder /app/src-web /app/src-web

# # Koyeb uses this
# ENV PORT=8000

# ENV RUST_LOG=debug
# ENV RUST_BACKTRACE=1

# EXPOSE 8000

CMD ["/app/choredom-website"]
