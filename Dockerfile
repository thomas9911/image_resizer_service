FROM rust:latest

RUN apt update
RUN apt install -y protobuf-compiler
WORKDIR /app
COPY build.rs Cargo.lock Cargo.toml ./
COPY src src
COPY proto proto

RUN cargo build --release
CMD cargo run --release
