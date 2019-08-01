FROM rustlang/rust:nightly

RUN cd && rustup default nightly
WORKDIR /app
ADD . /app
RUN cargo build --release
EXPOSE 8080
CMD [ "./target/release/actix_hello_world" ]