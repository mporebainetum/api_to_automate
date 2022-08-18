# syntax=docker/dockerfile:1
FROM rust:1.62 as builder
WORKDIR /usr/src/api_to_automate
COPY . .
RUN apt update && apt install musl-tools -y
RUN cargo test
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl

FROM alpine:latest
WORKDIR /root/
COPY --from=builder /usr/src/api_to_automate/target/x86_64-unknown-linux-musl/debug/api_to_automate /root/api_to_automate
#ENTRYPOINT ["tail"]
#CMD ["-f", "/dev/null"]
CMD ["/root/api_to_automate"]
