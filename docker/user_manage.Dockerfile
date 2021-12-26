FROM suxiaoshao/rust as builder
COPY ./server /app/mini-oss
RUN cd /app/mini-oss/packages/user_manage \
    && cargo build --release \
    && strip /app/mini-oss/target/x86_64-unknown-linux-musl/release/user_manage
FROM scratch as prod
COPY --from=builder ./app/mini-oss/target/x86_64-unknown-linux-musl/release/user_manage .
EXPOSE 80
CMD [ "./user_manage" ]