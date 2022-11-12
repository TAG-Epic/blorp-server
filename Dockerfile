FROM rust:latest
WORKDIR bot
COPY . ./
RUN cargo b --release
CMD ["./target/release/blorp-server"]
