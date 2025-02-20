FROM rust:1.83-slim-bookworm

RUN apt-get update && apt-get upgrade -y && apt-get clean && apt-get autoremove

COPY ./ ./

WORKDIR /src/.

RUN cargo run
# CMD ["./target/release/aoc"]