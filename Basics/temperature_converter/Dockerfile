FROM rust:latest

WORKDIR /temperature_converter

COPY .  /temperature_converter

RUN cargo build --release

CMD ["./target/release/temperature_converter"]
