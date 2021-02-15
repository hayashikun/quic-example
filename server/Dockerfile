FROM rust:slim-buster

WORKDIR /root/quic-example
COPY .. .
RUN cargo build --release
EXPOSE 8080
CMD target/release/quic-example
