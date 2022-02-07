FROM suxiaoshao/rust as builder
COPY ./server /app/mini-oss
RUN cd /app/mini-oss/packages/open \
    && cargo build --release \
    && strip /app/mini-oss/target/x86_64-unknown-linux-musl/release/open
FROM scratch as prod
COPY --from=builder ./app/mini-oss/target/x86_64-unknown-linux-musl/release/open .
EXPOSE 80
CMD [ "./open" ]