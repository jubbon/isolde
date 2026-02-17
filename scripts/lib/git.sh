#!/bin/bash
#
# Git-related functions for isolde.sh
#
# Note: This script is sourced by isolde.sh
# SCRIPT_DIR and utility functions are already available

# Initialize project git repository
init_project_repo() {
    local project_dir="$1"
    local project_name="$2"
    local project_repo_dir="$project_dir/project"

    # Skip if already a git repository
    if is_git_repository "$project_repo_dir"; then
        log_debug "Project git repository already exists"
        return 0
    fi

    # Create directory if it doesn't exist
    mkdir -p "$project_repo_dir" || {
        log_error "Failed to create project directory"
        return 1
    }

    # Initialize git repository
    git -C "$project_repo_dir" init -q || {
        log_error "Failed to initialize git repository"
        return 1
    }

    # Create initial README
    cat > "$project_dir/project/README.md" << EOF
# $project_name

This project was created using the Claude Code devcontainer template system.

## Getting Started

1. Open this project in VS Code
2. Reopen in Container when prompted
3. Start coding!

## Project Structure

- \`project/\` - Your main project code (this directory)
- \`.devcontainer/\` - Devcontainer configuration
- \`.claude/\` - Claude Code configuration (not in git)

## DevContainer

This project uses a devcontainer for development. The configuration is in the
\`.devcontainer/\` directory (a separate git repository).

To rebuild the container:
1. Press F1 in VS Code
2. Select "Dev Containers: Rebuild Container"
EOF

    # Add and commit
    git -C "$project_dir/project" add README.md
    git -C "$project_dir/project" commit -m "Initial commit" -q

    log_debug "Initialized project git repository"
}

# Initialize devcontainer git repository
init_devcontainer_repo() {
    local project_dir="$1"
    local project_name="$2"
    local devcontainer_dir="$project_dir/.devcontainer"

    # Skip if already a git repository with commits
    if is_git_repository "$devcontainer_dir"; then
        # Check if there are already commits
        if git -C "$devcontainer_dir" rev-parse --git-config >/dev/null 2>&1 && \
           git -C "$devcontainer_dir" rev-parse HEAD >/dev/null 2>&1; then
            log_debug "Devcontainer git repository already initialized with commits"
            return 0
        fi
    fi

    # Create directory if it doesn't exist
    mkdir -p "$devcontainer_dir" || {
        log_error "Failed to create .devcontainer directory"
        return 1
    }

    # Initialize git repository (safe to run if already initialized)
    git -C "$devcontainer_dir" init -q 2>/dev/null || {
        log_error "Failed to initialize devcontainer git repository"
        return 1
    }

    # Create README
    cat > "$project_dir/.devcontainer/README.md" << EOF
# DevContainer Configuration for $project_name

This repository contains the devcontainer configuration for the project.

## Structure

- \`devcontainer.json\` - Main devcontainer configuration
- \`Dockerfile\` - Container image definition
- \`features/\` - Devcontainer features (symlinks to template repository)

## Features

The following features are included:
- \`claude-code\` - Claude Code CLI installation
- \`proxy\` - HTTP proxy configuration (if enabled)

## Updates

To update the devcontainer configuration:
1. Pull the latest from the template repository
2. Rebuild the container

For more information, see the template repository documentation.
EOF

    # Create .gitignore for Claude-specific files
    cat > "$project_dir/.devcontainer/.gitignore" << EOF
# Claude Code local files (not in git)
.claude/
settings.json

# OMC state files (not in git)
.omc/

# IDE files
.vscode/
.idea/
EOF

    # Add all files and commit
    # Use add -A to include all files (including those created by apply_template)
    git -C "$project_dir/.devcontainer" add -A
    git -C "$project_dir/.devcontainer" commit -m "Initial devcontainer setup" -q

    log_debug "Initialized devcontainer git repository"
}

# Set remote repository URLs
set_git_remote() {
    local repo_dir="$1"
    local remote_url="$2"

    if [ -z "$remote_url" ]; then
        return 0
    fi

    git -C "$repo_dir" remote add origin "$remote_url" 2>/dev/null || {
        git -C "$repo_dir" remote set-url origin "$remote_url" || {
            log_warn "Failed to set git remote"
            return 1
        }
    }

    log_debug "Set git remote: $remote_url"
}

# Create .gitignore for project
create_project_gitignore() {
    local project_dir="$1"

    cat > "$project_dir/project/.gitignore" << EOF
# Claude Code local files
.claude/

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Python
__pycache__/
*.py[cod]
*\$py.class
*.so
.Python
venv/
env/
.venv/

# Node
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Rust
target/
Cargo.lock

# Go
*.sum
*.test
*.prof
EOF

    git -C "$project_dir/project" add .gitignore
    git -C "$project_dir/project" commit -m "Add .gitignore" -q

    log_debug "Created .gitignore"
}

# Check if directory is a git repository
is_git_repository() {
    local dir="$1"
    [ -d "$dir/.git" ]
}

# Get git status (for verification)
get_git_status() {
    local dir="$1"
    git -C "$dir" status --short 2>/dev/null
}

# Verify git repositories were created correctly
verify_git_repos() {
    local project_dir="$1"
    local errors=0

    # Check project repository
    if ! is_git_repository "$project_dir/project"; then
        log_error "Project git repository not found"
        ((errors++))
    fi

    # Check devcontainer repository
    if ! is_git_repository "$project_dir/.devcontainer"; then
        log_error "Devcontainer git repository not found"
        ((errors++))
    fi

    return $errors
}
