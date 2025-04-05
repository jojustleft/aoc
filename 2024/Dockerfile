FROM rust:1.83-slim-bookworm

RUN apt-get update && apt-get upgrade -y && apt-get clean && apt-get autoremove

# Cache build dependencies
RUN mkdir src
RUN echo "fn main() {}" >> src/main.rs
COPY Cargo.lock ./
COPY Cargo.toml ./
RUN cargo build --release
RUN rm src/main.rs

# AOC
COPY ./ ./
WORKDIR /src/.
RUN cargo run
# CMD ["./target/release/aoc"]