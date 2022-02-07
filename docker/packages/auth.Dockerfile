FROM suxiaoshao/rust as builder
COPY ./server /app/mini-oss
RUN cd /app/mini-oss/packages/auth \
    && cargo build --release \
    && strip /app/mini-oss/target/x86_64-unknown-linux-musl/release/auth
FROM scratch as prod
COPY --from=builder ./app/mini-oss/target/x86_64-unknown-linux-musl/release/auth .
EXPOSE 80
CMD [ "./auth" ]