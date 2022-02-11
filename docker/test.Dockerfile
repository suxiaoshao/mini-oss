FROM suxiaoshao/rust:latest
COPY ./server /app/mini-oss
COPY ./server/.cargo/config /usr/local/cargo
RUN cargo install cargo-watch