FROM rust
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
RUN apt-get update \
    && apt-get install -y musl-tools \
    && rustup component add rustfmt \
    && rustup target add x86_64-unknown-linux-musl