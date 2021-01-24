FROM rust:1.40 as builder
WORKDIR /usr/src/rust-sandbox
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/helloworld /usr/local/bin/helloworld
CMD ["helloworld"]