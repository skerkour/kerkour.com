FROM golang:1.22 AS go

# Final image
FROM debian:12

# set up timezone: UTC
ENV TZ="Europe/London"
# ENV LC_ALL="en_US.UTF-8"
ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US.UTF-8"

# Install dependencies
ENV DEBIAN_FRONTEND noninteractive
RUN apt update && apt upgrade -y
RUN apt install -y git ca-certificates make vim \
    bash gcc g++ zsh curl wget zip coreutils sqlite3 python3 gnupg xz-utils bsdextrautils ssh zip

RUN update-ca-certificates

# setup node
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
RUN apt update && apt install -y nodejs
RUN npm install -g npm@latest

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

EXPOSE 8082
