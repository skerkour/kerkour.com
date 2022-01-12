FROM ubuntu:latest

RUN apt update && apt upgrade -y

RUN apt install -y gcc curl wget git vim zip sqlite3 libdigest-sha-perl zsh build-essential make

ARG USERNAME=skerkour
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN adduser \
   --system \
   --shell /bin/bash \
   --gecos $USERNAME \
   --group \
   --disabled-password \
   --home /home/$USERNAME \
   $USERNAME


# Set up Rust
USER $USERNAME

COPY bashrc /home/$USERNAME/.bashrc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

USER root

EXPOSE 8000 8080
