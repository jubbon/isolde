"""Behave environment setup/teardown.

This module provides hooks for setting up and tearing down the test environment
for E2E testing, including workspace management and container cleanup.
"""

import tempfile
import shutil
import subprocess
import os
import sys
import time

# Add support directory to path for importing helpers
sys.path.insert(0, os.path.abspath(os.path.dirname(__file__)))

try:
    from helpers import ContainerHelper, FileHelper, cleanup_test_resources
    HELPERS_AVAILABLE = True
except ImportError:
    HELPERS_AVAILABLE = False


def before_all(context):
    """Global setup before all scenarios."""
    # Ensure isolde is installed
    result = subprocess.run(
        ["which", "isolde"],
        capture_output=True
    )
    if result.returncode != 0:
        raise RuntimeError("isolde CLI not found. Run 'make install' first.")

    # Check if devcontainers CLI is available
    devc_result = subprocess.run(
        ["which", "devcontainer"],
        capture_output=True
    )
    context.devcontainers_available = devc_result.returncode == 0

    # Check if Docker is running
    docker_result = subprocess.run(
        ["docker", "info"],
        capture_output=True
    )
    context.docker_available = docker_result.returncode == 0

    if not context.docker_available:
        raise RuntimeError("Docker is not running. Start Docker daemon for E2E tests.")

    # Pre-pull base images to speed up tests
    base_images = [
        "mcr.microsoft.com/devcontainers/base:ubuntu",
        "mcr.microsoft.com/devcontainers/python:3.12",
        "mcr.microsoft.com/devcontainers/node:22"
    ]

    for image in base_images:
        subprocess.run(
            ["docker", "pull", image],
            capture_output=True,
            timeout=300
        )


def before_scenario(context, scenario):
    """Setup workspace before each scenario."""
    # Create unique workspace for this scenario
    scenario_id = scenario.name.lower().replace(' ', '_')[:30]
    context.test_workspace = tempfile.mkdtemp(prefix=f"e2e-{scenario_id}-")
    context.test_images = []
    context.test_containers = []

    # Track project name and path
    context.project_name = None
    context.test_project_path = None

    # Track command results
    context.last_exit_code = None
    context.last_output = None

    # Initialize generator from generators module
    try:
        from generators import get_generator
        context.generator = get_generator("shell-script")
    except ImportError:
        context.generator = None


def after_scenario(context, scenario):
    """Cleanup after each scenario."""
    # Stop any running containers first
    if HELPERS_AVAILABLE:
        # Stop containers associated with the project
        if hasattr(context, 'project_name') and context.project_name:
            ContainerHelper.remove_container(context.project_name, force=True)

        # Clean up any tracked containers
        for container in getattr(context, 'test_containers', []):
            try:
                subprocess.run(
                    ["docker", "rm", "-f", container],
                    capture_output=True,
                    timeout=30
                )
            except Exception:
                pass
    else:
        # Fallback cleanup without helpers
        if hasattr(context, 'project_name') and context.project_name:
            try:
                # Try to find and stop containers by project name
                result = subprocess.run(
                    ["docker", "ps", "-q", "--filter", f"name={context.project_name}"],
                    capture_output=True,
                    text=True
                )
                for container_id in result.stdout.strip().split('\n'):
                    if container_id:
                        subprocess.run(
                            ["docker", "stop", container_id],
                            capture_output=True,
                            timeout=30
                        )
                        subprocess.run(
                            ["docker", "rm", container_id],
                            capture_output=True,
                            timeout=30
                        )
            except Exception:
                pass

    # Remove test images
    for image in getattr(context, 'test_images', []):
        try:
            subprocess.run(
                f"docker rmi {image} -f",
                shell=True,
                capture_output=True,
                timeout=60
            )
        except Exception:
            pass

    # Remove workspace directory
    if hasattr(context, 'test_workspace'):
        try:
            shutil.rmtree(context.test_workspace, ignore_errors=True)
        except Exception:
            pass

    # Clean up any custom image tags created during tests
    if hasattr(context, 'custom_image_name'):
        try:
            subprocess.run(
                f"docker rmi {context.custom_image_name} -f",
                shell=True,
                capture_output=True,
                timeout=60
            )
        except Exception:
            pass


def after_all(context):
    """Global cleanup after all scenarios."""
    # Clean up any orphaned test containers
    try:
        result = subprocess.run(
            ["docker", "ps", "-q", "--filter", "label=e2e-test"],
            capture_output=True,
            text=True
        )
        for container_id in result.stdout.strip().split('\n'):
            if container_id:
                subprocess.run(
                    ["docker", "rm", "-f", container_id],
                    capture_output=True,
                    timeout=30
                )
    except Exception:
        pass
