FROM rust:1.74.0
WORKDIR /usr/src/rusty-llama
COPY . .
RUN cargo install --path .
CMD ["rusty-llama"]