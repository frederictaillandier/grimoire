FROM docker.io/library/rust:slim-bookworm as builder
WORKDIR /usr/src/app
RUN cargo new --bin grimoire
WORKDIR /usr/src/app/grimoire
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release && \
    rm src/*.rs && \
    rm target/release/deps/grimoire*
COPY src ./src
RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/grimoire/target/release/grimoire /app/grimoire
CMD ["/app/grimoire"]
