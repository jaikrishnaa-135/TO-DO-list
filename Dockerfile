# Use the official Rust image
FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build --release

# Final image
FROM debian:bullseye-slim

# Install dependencies for Actix and SQLx
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/your_binary_name /usr/local/bin/wasm_project
COPY ./static /static

ENV ROCKET_ENV=production
EXPOSE 8080

CMD ["your_binary_name"]
