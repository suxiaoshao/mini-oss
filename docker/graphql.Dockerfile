FROM rust as builder
COPY ./server /app/novel
RUN cd /app/novel/graphql \
    && rustup component add rustfmt \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release
FROM scratch as prod
COPY --from=builder ./app/novel/target/x86_64-unknown-linux-musl/release/graphql .
CMD [ "./graphql" ]