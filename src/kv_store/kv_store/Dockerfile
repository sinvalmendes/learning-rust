FROM rustlang/rust:nightly

RUN cd && rustup default nightly
WORKDIR /app
ADD . /app
RUN cargo build --release
EXPOSE 8000
CMD [ "./target/release/kv_store" ]