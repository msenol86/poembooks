# 1. This tells docker to use the Rust official image
FROM rust:1.84

WORKDIR /home/poembooks

ENV PG_HOST="localpgsql"

# 2. Copy the files in your machine to the Docker image
COPY ["./", "./"]

# Build your program for release
RUN cargo build --release && cp ./target/release/poembooks ./poembooks && chmod +x ./poembooks  && rm -rf ./target

EXPOSE 3000

HEALTHCHECK --interval=2m --timeout=5s \
  CMD curl -f http://localhost:3000/health  || exit 1


# Run the binary
CMD ["./poembooks"]
# CMD ["sleep infinity"]
