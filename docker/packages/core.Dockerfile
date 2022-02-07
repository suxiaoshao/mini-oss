FROM suxiaoshao/rust as builder
COPY ./server /app/mini-oss
RUN cd /app/mini-oss/packages/core \
    && cargo build --release \
    && strip /app/mini-oss/target/x86_64-unknown-linux-musl/release/core
FROM scratch as prod
COPY --from=builder ./app/mini-oss/target/x86_64-unknown-linux-musl/release/core .
EXPOSE 80
CMD [ "./core" ]