"""Behave environment setup/teardown."""

import tempfile
import shutil
import subprocess
import sys
import os

# Add support directory to path for imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'support')))

# Global container pool for reusing containers across scenarios
# Key: template name, Value: (container_name, image_name)
CONTAINER_POOL = {}

# Global image registry for tracking built images
# Key: project_name, Value: image_name
IMAGE_REGISTRY = {}


def before_scenario(context, scenario):
    """Setup workspace before each scenario."""
    context.test_workspace = tempfile.mkdtemp(prefix="e2e-")
    context.test_images = []
    context.test_containers = []

    # Expose global pools to context for step definitions
    context.CONTAINER_POOL = CONTAINER_POOL
    context.IMAGE_REGISTRY = IMAGE_REGISTRY


def after_scenario(context, scenario):
    """Cleanup after each scenario."""
    # Stop and remove test containers
    for container in getattr(context, 'test_containers', []):
        subprocess.run(f"docker stop {container}", shell=True, capture_output=True)
        subprocess.run(f"docker rm {container}", shell=True, capture_output=True)

    # Remove test images
    for image in getattr(context, 'test_images', []):
        subprocess.run(f"docker rmi {image} -f", shell=True, capture_output=True)

    # Remove workspace
    if hasattr(context, 'test_workspace'):
        shutil.rmtree(context.test_workspace, ignore_errors=True)


def before_all(context):
    """Setup before all scenarios run."""
    # Ensure clean state at start
    _cleanup_orphaned_containers()
    _cleanup_orphaned_images()


def after_all(context):
    """Cleanup after all scenarios complete."""
    # Clean up any remaining pooled containers
    for template, (container_name, _) in list(CONTAINER_POOL.items()):
        subprocess.run(f"docker stop {container_name}", shell=True, capture_output=True)
        subprocess.run(f"docker rm {container_name}", shell=True, capture_output=True)
    CONTAINER_POOL.clear()

    # Clean up any remaining images
    for image in IMAGE_REGISTRY.values():
        subprocess.run(f"docker rmi {image} -f", shell=True, capture_output=True)
    IMAGE_REGISTRY.clear()


def _cleanup_orphaned_containers():
    """Remove any orphaned e2e test containers."""
    result = subprocess.run(
        "docker ps -a --filter 'name=e2e-' --format '{{{{.Names}}}}'",
        shell=True, capture_output=True, text=True
    )
    if result.returncode == 0 and result.stdout.strip():
        for container in result.stdout.strip().split('\n'):
            subprocess.run(f"docker rm -f {container}", shell=True, capture_output=True)


def _cleanup_orphaned_images():
    """Remove any orphaned e2e test images."""
    result = subprocess.run(
        "docker images --format '{{{{.Repository}}}:{{{{.Tag}}}}' | grep '^e2e-'",
        shell=True, capture_output=True, text=True
    )
    if result.returncode == 0 and result.stdout.strip():
        for image in result.stdout.strip().split('\n'):
            subprocess.run(f"docker rmi -f {image}", shell=True, capture_output=True)
