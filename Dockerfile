# Development container for local development & Gitlab-CI.
FROM rust:1.76.0 AS detmap_dev

ENV DEFAULT_USER=detmap
ENV DEBIAN_FRONTEND=noninteractive
RUN \
    apt-get update && \
    apt-get install -yq --no-install-recommends \
    # Docker, only used to build/test this dockerfile.
    docker.io docker-compose

# Add rustfmt
RUN rustup component add rustfmt clippy

# User setup
RUN useradd -l -G docker -md /home/$DEFAULT_USER -s /bin/bash $DEFAULT_USER
ENV HOME=/home/$DEFAULT_USER
WORKDIR $HOME
USER $DEFAULT_USER
