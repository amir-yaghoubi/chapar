FROM clux/muslrust:stable AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM clux/muslrust:stable AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json


FROM clux/muslrust:stable AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --bin run --release --target x86_64-unknown-linux-musl


# Need cacerts
# Run stage
FROM gcr.io/distroless/static:nonroot

COPY --from=builder --chown=nonroot:nonroot /volume/target/x86_64-unknown-linux-musl/release/service /app/chapar-service

WORKDIR /app

ENTRYPOINT ["/chapar-service"]