+++
date = 2022-01-12T10:00:00Z
title = "Secure and immutable development environments with Dev Containers"
type = "post"
tags = ["hacking", "security", "rust", "programming", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/secure-programming-with-vscode-dev-containers"

[extra]
lang = "en"

comment ="""
"""
+++

*"But, it works on my machine"*

*"If I install this NPM package, is it going to `rm -rf /` or infect me with a cryptominner?"*

*"I spent the whole day trying to install $DATABASE on my laptop but it still doesn't work!"*



As [we saw 2 months ago](/rust-crate-backdoor), supply chain attacks are more and more common, and their frequency is only going to increase because backdooring dependencies is easy, and detecting a backdoor in an ocean of third-party code is hard.

At the end of [the article](/rust-crate-backdoor), I recommend using cloud sandboxed development environments such as GitHub CodeSpaces or GitPod. These solutions may not fit your requirements or your taste because you don't like GitHub, prefer to develop locally, have a spotty internet connection, or whatever.


Today, we are going to learn how to mitigate the impact of supply chain attacks and how to create immutable local development environments: everybody working on a project is going to have the exact same environment as everyone else, no more impossible to reproduce bugs or time lost fighting environment-related bugs.



## Immutable Developments Environments


An immutable Developments Environment is a setup where the developers working on a project have the exact same development environment: same OS, same packages, same tools.

A developer can't install a particular package in the environment. As we are going to see, if a developer wants to install a package, they have to edit a configuration file, make it accepted in code review, and then everyone will have this package installed.

It allows new developers to start contributing to the project in a blink: `git pull` and we are ready to code.


## Sandboxed Developments Environments

"Modern" (whatever it means) software projects rely on hundreds to thousands of external dependencies maintained by hundreds to thousands of different individuals and organizations.

If any of these maintainers sell or give away ownership of their packages, get compromised, or simply decide to go rogue, all their dependents (upstream projects) are compromised.

On a typical developer machine, code execution means complete compromise, with the ability to extract sensitive data such as API Keys, personal files, or browsers' cookies.

This is why we need to sandbox each project, so the impact of a compromised dependency is greatly reduced.


## Introducing Development Containers

Visual Studio Code lets you sandbox your projects with Development containers.

![VS Code Dev Containers](/2022/secure-programming-with-vscode-dev-containers/devcontainers.png)

For that, you need to download the official [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension.

Then, all the configuration files live in the `.devcontainer` in the root of the Git repository.

The first configuration file we need is:

**[.devcontainer/devcontainer.json](https://github.com/skerkour/kerkour.com/blob/main/.devcontainer/devcontainer.json)**
```json
{
  "dockerFile": "Dockerfile",
  "extensions": [
    "matklad.rust-analyzer"
  ],
  "forwardPorts": [
    8000,
    8080
  ],
  "containerUser": "skerkour"
}
```

With `devcontainer.json` we can specify which Dockerfile to use to build the container, which VS Code extension to install for the project (they won't be installed globally), which ports to open, and [many other things](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

The second file we need is a `Dockerfile`:

**[.devcontainer/Dockerfile](https://github.com/skerkour/kerkour.com/blob/main/.devcontainer/Dockerfile)**
```dockerfile
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
```

The Dockerfile is the recipe for our immutable development environment. This is where all our packages and tools are installed. Here, for example, we install a Rust toolchain and a few Unix tools for debugging.

As you can see, we create an unprivileged user `skerkour`, but at the end, we use `USER root`. Why?

Because when running the container, VS Code needs to install some helper tools to allow development in the container, and they require root access. This is why we use `"containerUser": "skerkour"` in `devcontainer.json` so the development indeed happens with the unprivileged user.


Then, we can add a custom configuration file. Here, I chose to add a `.bashrc` file in order to have some Git shortcuts:

**[.devcontainer/bashrc](https://github.com/skerkour/kerkour.com/blob/main/.devcontainer/bashrc)**
```bash
alias gs="git status"
alias ga="git add"
alias gu="git add -u"
alias gm="git commit -m"
alias gp="git push"


source $HOME/.cargo/env
```

Finally, launch the command palette (Ctrl/Cmd + Maj + P) and run: `Remote-Containers: Rebuild Container`.

That's all!

VS Code is now running you project in a container.

{{< subscribe_form >}}

## Some Closing Thoughts

Are dev containers 100% secure? No. Nothing is 100% secure, container escapes exist, and privilege escalation exploits too.

That being said, I believe that VS Code development containers are today the best solution to sandbox your development environments and reduce the impact of a supply chain compromise, all while providing an awesome onboarding experience for new developers, whether it be for an open-source project, or in a team.


You can find the detailed reference [in the official documentation](https://code.visualstudio.com/docs/remote/containers).


## The code is on GitHub

As always, the code is on GitHub: [github.com/skerkour/kerkour.com](https://github.com/skerkour/kerkour.com/tree/main/blog/.devcontainer) (please don't forget to star the repo 🙏).
