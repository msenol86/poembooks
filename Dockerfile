# 1. This tells docker to use the Rust official image
FROM rust:1.84

WORKDIR /home/poembooks

ENV PG_HOST="localpgsql"

# 2. Copy the files in your machine to the Docker image
COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml

# Build your program for release
RUN cargo build --release

EXPOSE 3000

HEALTHCHECK --interval=2m --timeout=5s \
  CMD curl -f http://localhost:3000/health  || exit 1


# ENTRYPOINT ["bash", "-c", "param2", "chmod +x /home/poembooks/target/release/poembooks && ./home/poembooks/target/release/poembooks"]

# Run the binary
CMD ["/bin/bash", "-c", "chmod +x /home/poembooks/target/release/poembooks && cd /home/poembooks/target/release/ && ./poembooks"]
# CMD ["sleep infinity"]


