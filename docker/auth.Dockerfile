FROM suxiaoshao/rust as builder
COPY ./server /app/novel
RUN cd /app/novel/packages/auth \
    && cargo build --release
FROM scratch as prod
COPY --from=builder ./app/novel/target/x86_64-unknown-linux-musl/release/auth .
EXPOSE 80
CMD [ "./auth" ]