"""Behave environment setup/teardown."""

import tempfile
import shutil
import subprocess
import os


def before_scenario(context, scenario):
    """Setup workspace before each scenario."""
    context.test_workspace = tempfile.mkdtemp(prefix="e2e-")
    context.test_images = []


def after_scenario(context, scenario):
    """Cleanup after each scenario."""
    # Remove test images
    for image in getattr(context, 'test_images', []):
        subprocess.run(f"docker rmi {image} -f", shell=True, capture_output=True)

    # Handle concurrent project cleanup
    if hasattr(context, 'concurrent_projects'):
        for name in context.concurrent_projects:
            # Try to remove images that might have been built for concurrent projects
            image_name = f"e2e-{name}"
            subprocess.run(f"docker rmi {image_name}* -f", shell=True, capture_output=True)

    # Remove workspace
    if hasattr(context, 'test_workspace'):
        shutil.rmtree(context.test_workspace, ignore_errors=True)
