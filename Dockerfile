FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN apt-get update -y && \
    apt-get install -y \
    git \
    curl \
    wget \
    unzip

ARG USERNAME=vscode
USER ${USERNAME}

# Add completions
RUN echo "source /usr/share/bash-completion/completions/git" >> /home/${USERNAME}/.bashrc
RUN echo "source <( rustup completions bash )" >> /home/${USERNAME}/.bashrc
RUN echo "source <( rustup completions bash cargo )" >> /home/${USERNAME}/.bashrc

RUN rustup component add rustfmt clippy
RUN cargo install just