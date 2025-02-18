# 1. This tells docker to use the Rust official image
FROM rust:1.84

WORKDIR /home/poembooks

# 2. Copy the files in your machine to the Docker image
COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml

# Build your program for release
RUN cargo build --release

EXPOSE 3000

# Run the binary
CMD ["./home/target/release/poembooks"]
# CMD ["sleep infinity"]


