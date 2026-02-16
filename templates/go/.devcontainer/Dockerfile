ARG BASE_IMAGE=debian:bookworm-slim
FROM ${BASE_IMAGE}

# User arguments with defaults
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=1000

# Set DEBIAN_FRONTEND for non-interactive apt
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    wget \
    vim \
    jq \
    build-essential \
    ca-certificates \
    gosu \
    && rm -rf /var/lib/apt/lists/*

# Install Go (latest version)
RUN GO_VERSION=$(curl -s https://go.dev/dl/?mode=json | grep -o '"version"[^,]*' | head -1 | cut -d'"' -f4) && \
    GO_VERSION=${GO_VERSION#go} && \
    wget -q https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz && \
    rm go${GO_VERSION}.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

WORKDIR /workspaces

# Pre-install sudo
RUN apt-get update && apt-get install -y sudo && rm -rf /var/lib/apt/lists/*

# Create user with sudo access
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USERNAME} --shell /bin/bash --create-home ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${USERNAME}:${USERNAME} /workspaces

USER ${USERNAME}
