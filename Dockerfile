FROM rust:1.80-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/bank_codes_api*

# Copy source code
COPY src ./src

# Build the actual application with static linking
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:latest

# Create a non-root user
RUN adduser -D -s /bin/sh app

WORKDIR /app

# Copy the statically linked binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bank_codes_api .

# Copy the data file
COPY data.json .

# Change ownership to the app user
RUN chown -R app:app /app

USER app

EXPOSE 3000

CMD ["./bank_codes_api"]