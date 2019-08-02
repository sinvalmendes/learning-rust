# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
# http://whitfin.io/speeding-up-rust-docker-builds/

# FROM rustlang/rust:nightly as cargo-build

# RUN cd && rustup default nightly
# WORKDIR /app
# ADD . /app
# RUN cargo build 
# EXPOSE 8080
# CMD [ "./target/debug/actix_hello_world" ]

# # ------------------------------------------------------------------------------
# # Final Stage
# # ------------------------------------------------------------------------------

# FROM alpine:latest
# RUN addgroup -g 1000 actix_hello_world
# RUN adduser -D -s /bin/sh -u 1000 -G actix_hello_world actix_hello_world
# WORKDIR /home/myapp/bin/
# COPY --from=cargo-build /app/target/debug/actix_hello_world .
# RUN chown actix_hello_world:actix_hello_world actix_hello_world
# USER actix_hello_world
# RUN ls
# RUN pwd
# CMD [ "./actix_hello_world" ]





FROM rust:latest as build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

# create a new empty shell project
WORKDIR /app
ADD . /app
RUN cargo build --target x86_64-unknown-linux-musl

# our final base
FROM alpine:latest
# copy the build artifact from the build stage
COPY --from=build /app/target/debug/actix_hello_world .
# set the startup command to run your binary
EXPOSE 8080
CMD ["./actix_hello_world"]