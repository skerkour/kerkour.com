FROM ubuntu:latest

RUN apt update && apt upgrade -y

RUN apt install -y gcc curl wget git vim zip sqlite3 libdigest-sha-perl zsh build-essential make

ARG USER=skerkour

RUN adduser \
   --system \
   --shell /bin/bash \
   --gecos $USER \
   --group \
   --disabled-password \
   --home /home/$USER \
   $USER


# Set up Rust
USER $USER

COPY bashrc /home/$USER/.bashrc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

USER root

EXPOSE 8000 8080
