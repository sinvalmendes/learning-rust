# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
# http://whitfin.io/speeding-up-rust-docker-builds/

# base image to build the application
FROM rust:1.36 as build-image
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.toml
RUN pwd
RUN ls -la

RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/actix_hello_world*

RUN pwd
RUN ls -la

COPY . .

RUN pwd
RUN ls -la
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# runtime image with only the binary built by the previous image
FROM alpine:3.10
RUN addgroup -g 1000 app
RUN adduser -D -s /bin/sh -u 1000 -G app app
WORKDIR /home/app/bin/
COPY --from=build-image /app/target/x86_64-unknown-linux-musl/release/actix_hello_world .
RUN chown app:app actix_hello_world
USER app
EXPOSE 8080
CMD ["./actix_hello_world"]