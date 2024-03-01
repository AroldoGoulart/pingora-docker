FROM rust:latest

WORKDIR /usr/src/pingora

COPY . .

RUN apt-get update && apt-get install -y \
    cmake zlib1g-dev

RUN cargo build --release

CMD ["./target/release/pingora", "-c", "conf.yaml", "-d"]