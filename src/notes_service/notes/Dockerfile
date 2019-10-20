FROM rust:1.35

RUN cd && rustup default stable
WORKDIR /app
ADD . /app
RUN cargo build --release
EXPOSE 8000
CMD [ "./target/release/notes_service" ]