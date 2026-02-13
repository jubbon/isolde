ARG BASE_IMAGE=debian:bookworm-slim
FROM ${BASE_IMAGE}

# User arguments with defaults
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=1000

# Set DEBIAN_FRONTEND for non-interactive apt
# Note: Proxy variables (HTTP_PROXY, HTTPS_PROXY, NO_PROXY) are passed as ARG
# but NOT set as ENV to avoid affecting apt-get during build.
# They will be used only by the claude-code feature install script.
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

WORKDIR /workspaces

# Pre-install sudo to avoid conffile prompts later
RUN apt-get update && apt-get install -y sudo && rm -rf /var/lib/apt/lists/*

# Create user with sudo access
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USERNAME} --shell /bin/bash --create-home ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${USERNAME}:${USERNAME} /workspaces

# Switch to user
USER ${USERNAME}
