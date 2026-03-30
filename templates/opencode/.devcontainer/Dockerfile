ARG BASE_IMAGE=mcr.microsoft.com/devcontainers/base:ubuntu
FROM ${BASE_IMAGE}

# User arguments with defaults
ARG USERNAME=user

# Set DEBIAN_FRONTEND for non-interactive apt
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /workspaces

# If the base image has a 'vscode' user at UID 1000 and the requested user is different,
# reassign vscode to a high UID so that updateRemoteUserUID can give UID 1000 to USERNAME.
# This ensures bind-mounted host directories (owned by UID 1000) appear with the correct
# owner inside the container without needing chown on the bind mount.
RUN if [ "${USERNAME}" != "vscode" ] && id vscode >/dev/null 2>&1; then \
        userdel -r vscode 2>/dev/null || true; \
        groupdel vscode 2>/dev/null || true; \
    fi
