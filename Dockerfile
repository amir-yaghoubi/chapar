FROM messense/rust-musl-cross:x86_64-musl as builder
RUN apt-get update && apt-get install -y python3 && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/service /app/chapar-service
WORKDIR /app

CMD ["./chapar-service"]