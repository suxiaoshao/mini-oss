FROM rust as builder
COPY ./server /app/novel
RUN cd /app/novel/packages/auth \
    && apt-get update \
    && apt-get install -y musl-tools \
    && rustup component add rustfmt \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release
FROM scratch as prod
COPY --from=builder ./app/novel/target/x86_64-unknown-linux-musl/release/auth .
CMD [ "./auth" ]