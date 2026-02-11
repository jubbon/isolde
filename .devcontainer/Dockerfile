ARG BASE_IMAGE=debian:bookworm-slim
FROM ${BASE_IMAGE}

# Proxy arguments for build time
ARG HTTP_PROXY
ARG HTTPS_PROXY
ARG NO_PROXY

# User arguments with defaults
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=1000

# Set proxy environment variables for build time
ENV HTTP_PROXY=${HTTP_PROXY} \
    HTTPS_PROXY=${HTTPS_PROXY} \
    NO_PROXY=${NO_PROXY} \
    DEBIAN_FRONTEND=noninteractive

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

WORKDIR /workspaces

# Pre-install sudo to avoid conffile prompts later
RUN apt-get update && apt-get install -y sudo python3.11 python3-pip python3-venv && rm -rf /var/lib/apt/lists/*

# Create user with sudo access
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USERNAME} --shell /bin/bash --create-home ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${USERNAME}:${USERNAME} /workspaces

# Switch to user
USER ${USERNAME}
