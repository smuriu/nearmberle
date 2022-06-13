FROM rust:1.61.0

LABEL description="Container for builds"

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

COPY dev.sh meta.env /tmp/scripts/

RUN rustup default 1.61.0
RUN rustup target add wasm32-unknown-unknown

RUN apt-get -y update && apt-get install -y \
  git \
  less \
  vim \
  llvm \
  clang \
  locales sudo curl ca-certificates gcc libc6-dev python3-minimal acl \
  && rm -rf /var/lib/apt/lists/*
  
RUN bash /tmp/scripts/dev.sh "${USERNAME}" "${USER_UID}" "${USER_GID}"
