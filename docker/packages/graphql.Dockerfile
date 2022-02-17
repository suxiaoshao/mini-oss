FROM suxiaoshao/rust as builder
COPY ./server /app/mini-oss
COPY ./docker/packages/.cargo /app/mini-oss/.cargo
RUN cd /app/mini-oss/packages/graphql \
    && cargo build --release \
    && strip /app/mini-oss/target/x86_64-unknown-linux-musl/release/graphql
FROM scratch as prod
COPY --from=builder ./app/mini-oss/target/x86_64-unknown-linux-musl/release/graphql .
EXPOSE 80
CMD [ "./graphql" ]