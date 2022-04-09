FROM rust:1.60

RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get update \
    && rustup component add rustfmt