# -*- coding: utf-8 -*-
"""Step definitions for Layer 3 verification with container pooling."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then, when
import subprocess
import yaml
import time
from typing import Dict, List, Any


# Container pool for reusing containers across verification steps
# Key: template name, Value: (container_name, image_name)
CONTAINER_POOL: Dict[str, tuple] = {}

# Image registry tracking built images
# Key: project_name, Value: image_name
IMAGE_REGISTRY: Dict[str, str] = {}


def load_verification_checks(check_type: str) -> List[Dict[str, Any]]:
    """
    Load verification checks from YAML files in verification/ directory.

    Args:
        check_type: Type of checks to load (e.g., 'runtime', 'build', 'features')

    Returns:
        List of check dictionaries with keys: name, command, expected_output, exit_code
    """
    verification_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        'verification'
    )

    yaml_file = os.path.join(verification_dir, f'{check_type}.yaml')

    if not os.path.exists(yaml_file):
        # Return default checks if file doesn't exist
        return get_default_checks(check_type)

    with open(yaml_file, 'r') as f:
        data = yaml.safe_load(f)

    return data.get('checks', [])


def get_default_checks(check_type: str) -> List[Dict[str, Any]]:
    """Get default verification checks when YAML files don't exist."""
    defaults = {
        'runtime': [
            {'name': 'container_running', 'command': 'echo "running"', 'exit_code': 0}
        ],
        'build': [
            {'name': 'build_success', 'command': 'echo "built"', 'exit_code': 0}
        ],
        'features': [
            {'name': 'basic_check', 'command': 'which sh', 'exit_code': 0}
        ]
    }
    return defaults.get(check_type, [])


def get_or_start_container(context, template: str) -> str:
    """
    Get or start a container for the given template using pooling.

    Args:
        context: Behave context object
        template: Template name (e.g., 'python', 'nodejs')

    Returns:
        Container name
    """
    global CONTAINER_POOL, IMAGE_REGISTRY

    # Check if container already exists in pool
    if template in CONTAINER_POOL:
        container_name, image_name = CONTAINER_POOL[template]

        # Verify container is still running
        result = subprocess.run(
            f"docker inspect -f '{{{{.State.Status}}}}' {container_name}",
            shell=True, capture_output=True, text=True
        )

        if result.returncode == 0 and 'running' in result.stdout:
            return container_name

        # Container not running, remove from pool
        del CONTAINER_POOL[template]

    # Need to create a new container
    # First check if we have a built image
    project_name = getattr(context, 'project_name', f'e2e-{template}')

    if project_name in IMAGE_REGISTRY:
        image_name = IMAGE_REGISTRY[project_name]
    else:
        # Build the image first
        image_name = f"e2e-{template}-{int(time.time())}"
        project_path = os.path.join(context.test_workspace, project_name, ".devcontainer")

        build_result = subprocess.run(
            f"docker build -t {image_name} {project_path}",
            shell=True, capture_output=True, text=True
        )

        if build_result.returncode != 0:
            raise AssertionError(f"Failed to build image for template {template}:\n{build_result.stderr}")

        # Register the image
        IMAGE_REGISTRY[project_name] = image_name

        # Add to cleanup list
        if not hasattr(context, 'test_images'):
            context.test_images = []
        context.test_images.append(image_name)

    # Start the container
    container_name = f"e2e-{template}-container-{int(time.time())}"

    run_result = subprocess.run(
        f"docker run -d --name {container_name} {image_name} sleep infinity",
        shell=True, capture_output=True, text=True
    )

    if run_result.returncode != 0:
        raise AssertionError(f"Failed to start container for template {template}:\n{run_result.stderr}")

    # Add to cleanup list
    if not hasattr(context, 'test_containers'):
        context.test_containers = []
    context.test_containers.append(container_name)

    # Store in pool
    CONTAINER_POOL[template] = (container_name, image_name)

    return container_name


@when('a running container for template "{template}" exists')
def step_running_container(context, template: str):
    """Ensure a running container exists for the given template."""
    container_name = get_or_start_container(context, template)
    context.verification_container = container_name
    context.verification_template = template


@when('I run "{check_type}" verification checks')
def step_run_verification(context, check_type: str):
    """Execute verification checks of the specified type."""
    if not hasattr(context, 'verification_container'):
        raise AssertionError("No container available for verification. Use 'a running container for template \"{template}\" exists' first.")

    container_name = context.verification_container
    checks = load_verification_checks(check_type)

    # Store results for later assertion
    context.verification_results = []

    for check in checks:
        check_name = check.get('name', 'unnamed')
        command = check.get('command', '')
        expected_exit_code = check.get('exit_code', 0)
        expected_output = check.get('expected_output', None)

        # Execute command in container
        result = subprocess.run(
            f"docker exec {container_name} {command}",
            shell=True, capture_output=True, text=True
        )

        # Determine if check passed
        passed = (
            result.returncode == expected_exit_code and
            (expected_output is None or expected_output in result.stdout)
        )

        context.verification_results.append({
            'name': check_name,
            'passed': passed,
            'exit_code': result.returncode,
            'expected_exit_code': expected_exit_code,
            'stdout': result.stdout,
            'stderr': result.stderr,
            'command': command
        })


@when('I run "{check_type}" verification checks on template "{template}"')
def step_run_verification_with_template(context, check_type: str, template: str):
    """Ensure container exists and run verification checks."""
    step_running_container(context, template)
    step_run_verification(context, check_type)


@then('all verification checks should pass')
def step_all_checks_pass(context):
    """Assert all verification checks passed."""
    if not hasattr(context, 'verification_results'):
        raise AssertionError("No verification results found. Run verification checks first.")

    failed_checks = [
        r for r in context.verification_results
        if not r['passed']
    ]

    if failed_checks:
        error_msg = "Verification checks failed:\n"
        for check in failed_checks:
            error_msg += f"\n  - {check['name']}:\n"
            error_msg += f"      Command: {check['command']}\n"
            error_msg += f"      Exit code: {check['exit_code']} (expected {check['expected_exit_code']})\n"
            if check['stderr']:
                error_msg += f"      Stderr: {check['stderr']}\n"
            if check['stdout']:
                error_msg += f"      Stdout: {check['stdout'][:200]}...\n"

        raise AssertionError(error_msg)


@then('at least {min_count:d} verification checks should pass')
def step_min_checks_pass(context, min_count: int):
    """Assert at least the minimum number of checks passed."""
    if not hasattr(context, 'verification_results'):
        raise AssertionError("No verification results found. Run verification checks first.")

    passed_count = sum(1 for r in context.verification_results if r['passed'])

    if passed_count < min_count:
        total = len(context.verification_results)
        raise AssertionError(
            f"Expected at least {min_count} checks to pass, but only {passed_count}/{total} passed."
        )


@then('the verification check "{check_name}" should pass')
def step_specific_check_passes(context, check_name: str):
    """Assert a specific verification check passed."""
    if not hasattr(context, 'verification_results'):
        raise AssertionError("No verification results found. Run verification checks first.")

    for result in context.verification_results:
        if result['name'] == check_name:
            if result['passed']:
                return
            else:
                raise AssertionError(
                    f"Check '{check_name}' failed:\n"
                    f"  Command: {result['command']}\n"
                    f"  Exit code: {result['exit_code']} (expected {result['expected_exit_code']})\n"
                    f"  Stderr: {result['stderr']}\n"
                    f"  Stdout: {result['stdout'][:200]}"
                )

    raise AssertionError(f"Check '{check_name}' not found in results.")


@when('I stop the pooled container for template "{template}"')
def step_stop_pooled_container(context, template: str):
    """Stop and remove a pooled container for testing reuse scenarios."""
    global CONTAINER_POOL

    if template in CONTAINER_POOL:
        container_name, _ = CONTAINER_POOL[template]

        subprocess.run(
            f"docker stop {container_name}",
            shell=True, capture_output=True
        )
        subprocess.run(
            f"docker rm {container_name}",
            shell=True, capture_output=True
        )

        del CONTAINER_POOL[template]


# Cleanup function to be called from environment.py
def cleanup_container_pool(containers: List[str] = None):
    """Cleanup containers from the pool."""
    global CONTAINER_POOL

    if containers:
        for container in containers:
            subprocess.run(f"docker stop {container}", shell=True, capture_output=True)
            subprocess.run(f"docker rm {container}", shell=True, capture_output=True)

    # Also clear the pool
    for template, (container_name, _) in list(CONTAINER_POOL.items()):
        subprocess.run(f"docker stop {container_name}", shell=True, capture_output=True)
        subprocess.run(f"docker rm {container_name}", shell=True, capture_output=True)

    CONTAINER_POOL.clear()
