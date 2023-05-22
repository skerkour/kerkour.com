FROM ubuntu:latest

ENV TZ="Europe/London"
ENV LC_ALL="en_US.UTF-8"
ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US.UTF-8"

RUN apt update && apt upgrade -y
RUN apt install -y git git-lfs ca-certificates make vim bash curl wget zip coreutils sqlite3 \
    gnupg xz-utils libimage-exiftool-perl

RUN update-ca-certificates

# Setup skerkour user
ARG USERNAME=skerkour
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN addgroup --gid $USER_GID $USERNAME \
    && adduser --uid $USER_UID --ingroup $USERNAME --disabled-password --shell /bin/bash --gecos "" $USERNAME

USER $USERNAME

COPY bashrc /home/$USERNAME/.bashrc
COPY vimrc /home/$USERNAME/.vimrc

WORKDIR /home/skerkour
