FROM golang:1.20 AS go

FROM ubuntu:latest

RUN apt update && apt upgrade -y
RUN apt install -y git ca-certificates make vim \
    bash gcc g++ zsh curl wget zip coreutils sqlite3 python3 gnupg xz-utils bsdextrautils ssh zip

RUN update-ca-certificates

# TODO: set up timezone: UTC

# Setup dev user
ARG USERNAME=dev
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN addgroup --gid $USER_GID $USERNAME \
    && adduser --uid $USER_UID --ingroup $USERNAME --disabled-password --shell /bin/bash --gecos "" $USERNAME

USER $USERNAME

COPY bashrc /home/$USERNAME/.bashrc
COPY vimrc /home/$USERNAME/.vimrc

# Setup go
RUN mkdir -p /home/$USERNAME/.local/gopath

COPY --from=go /usr/local/go /home/$USERNAME/.local/go

ENV GOROOT /home/$USERNAME/.local/go
ENV GOPATH /home/$USERNAME/.local/gopath
ENV PATH $PATH:$GOPATH/bin:$GOROOT/bin

WORKDIR /
