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

# Install Rust (rustup) for all users
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal && \
    /root/.cargo/bin/rustup default stable
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspaces

# Pre-install sudo
RUN apt-get update && apt-get install -y sudo && rm -rf /var/lib/apt/lists/*

# Create user with sudo access
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USERNAME} --shell /bin/bash --create-home ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${USERNAME}:${USERNAME} /workspaces

# Copy Rust installation to user home
RUN cp -r /root/.cargo /home/${USERNAME}/ && \
    chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}/.cargo

USER ${USERNAME}

# Add cargo to PATH for non-root user
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Set default toolchain for the non-root user
RUN rustup default stable
