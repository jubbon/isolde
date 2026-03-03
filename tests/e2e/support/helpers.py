# -*- coding: utf-8 -*-
"""Test helper utilities for E2E testing.

This module provides reusable helper classes and functions for
E2E testing, including container management, file operations,
and validation utilities.
"""

import subprocess
import time
import json
import os
from typing import List, Dict, Any, Optional


class ContainerHelper:
    """Helper class for Docker container operations."""

    @staticmethod
    def get_containers(project_name: Optional[str] = None, all: bool = False) -> List[Dict[str, str]]:
        """
        Get list of containers.

        Args:
            project_name: Optional filter by project name
            all: Include stopped containers

        Returns:
            List of container info dictionaries
        """
        cmd = ["docker", "ps", "--format", "json"]
        if all:
            cmd.append("-a")

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        if result.returncode != 0:
            return []

        containers = []
        for line in result.stdout.strip().split('\n'):
            if line:
                try:
                    container = json.loads(line)
                    if project_name is None or project_name.lower() in container.get('Names', '').lower():
                        containers.append(container)
                except json.JSONDecodeError:
                    continue

        return containers

    @staticmethod
    def get_container_name(project_name: str) -> Optional[str]:
        """
        Get the container name for a project.

        Args:
            project_name: Project name to search for

        Returns:
            Container name or None if not found
        """
        containers = ContainerHelper.get_containers(project_name)
        if containers:
            return containers[0].get('Names')
        return None

    @staticmethod
    def is_container_running(project_name: str) -> bool:
        """
        Check if container is running.

        Args:
            project_name: Project name to check

        Returns:
            True if container is running
        """
        containers = ContainerHelper.get_containers(project_name, all=False)
        return len(containers) > 0

    @staticmethod
    def is_container_stopped(project_name: str) -> bool:
        """
        Check if container exists but is stopped.

        Args:
            project_name: Project name to check

        Returns:
            True if container exists but is stopped
        """
        all_containers = ContainerHelper.get_containers(project_name, all=True)
        running_containers = ContainerHelper.get_containers(project_name, all=False)
        return len(all_containers) > len(running_containers) > 0

    @staticmethod
    def wait_for_container(project_name: str, timeout: int = 30, running: bool = True) -> bool:
        """
        Wait for container to start/stop.

        Args:
            project_name: Project name to wait for
            timeout: Maximum wait time in seconds
            running: Wait for running (True) or stopped (False) state

        Returns:
            True if container reached desired state
        """
        for _ in range(timeout):
            if running:
                if ContainerHelper.is_container_running(project_name):
                    return True
            else:
                if not ContainerHelper.is_container_running(project_name):
                    return True
            time.sleep(1)
        return False

    @staticmethod
    def execute_in_container(project_name: str, command: List[str],
                             timeout: int = 30) -> subprocess.CompletedProcess:
        """
        Execute command in container.

        Args:
            project_name: Project name
            command: Command to execute
            timeout: Command timeout

        Returns:
            CompletedProcess result
        """
        container_name = ContainerHelper.get_container_name(project_name)
        if not container_name:
            raise RuntimeError(f"No container found for project {project_name}")

        return subprocess.run(
            ["docker", "exec", container_name] + command,
            capture_output=True,
            text=True,
            timeout=timeout
        )

    @staticmethod
    def get_container_logs(project_name: str, tail: Optional[int] = None) -> str:
        """
        Get container logs.

        Args:
            project_name: Project name
            tail: Number of lines to get from end

        Returns:
            Log output
        """
        container_name = ContainerHelper.get_container_name(project_name)
        if not container_name:
            return ""

        cmd = ["docker", "logs", container_name]
        if tail:
            cmd.extend(["--tail", str(tail)])

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        return result.stdout

    @staticmethod
    def stop_container(project_name: str, force: bool = False) -> bool:
        """
        Stop a container.

        Args:
            project_name: Project name
            force: Force stop

        Returns:
            True if successful
        """
        container_name = ContainerHelper.get_container_name(project_name)
        if not container_name:
            return False

        cmd = ["docker", "stop"]
        if force:
            cmd.append("--force")  # Note: docker stop doesn't have --force, this is for isolde stop
        cmd.append(container_name)

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        return result.returncode == 0

    @staticmethod
    def remove_container(project_name: str, force: bool = True) -> bool:
        """
        Remove a container.

        Args:
            project_name: Project name
            force: Force removal

        Returns:
            True if successful
        """
        container_name = ContainerHelper.get_container_name(project_name, all=True)
        if not container_name:
            return False

        cmd = ["docker", "rm"]
        if force:
            cmd.append("-f")
        cmd.append(container_name)

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )

        return result.returncode == 0

    @staticmethod
    def image_exists(image_name: str) -> bool:
        """
        Check if Docker image exists.

        Args:
            image_name: Image name to check

        Returns:
            True if image exists
        """
        result = subprocess.run(
            ["docker", "images", "-q", image_name],
            capture_output=True,
            text=True,
            timeout=30
        )

        return len(result.stdout.strip()) > 0

    @staticmethod
    def get_image_tags() -> List[str]:
        """
        Get list of all Docker image tags.

        Returns:
            List of image tags
        """
        result = subprocess.run(
            ["docker", "images", "--format", "{{.Repository}}:{{.Tag}}"],
            capture_output=True,
            text=True,
            timeout=30
        )

        if result.returncode != 0:
            return []

        return [line.strip() for line in result.stdout.strip().split('\n') if line.strip()]

    @staticmethod
    def remove_image(image_name: str, force: bool = False) -> bool:
        """
        Remove a Docker image.

        Args:
            image_name: Image to remove
            force: Force removal

        Returns:
            True if successful
        """
        cmd = ["docker", "rmi"]
        if force:
            cmd.append("-f")
        cmd.append(image_name)

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=120
        )

        return result.returncode == 0


class FileHelper:
    """Helper class for file operations."""

    @staticmethod
    def read_json(path: str) -> Dict[str, Any]:
        """
        Read and parse JSON file.

        Args:
            path: Path to JSON file

        Returns:
            Parsed JSON data
        """
        with open(path, 'r') as f:
            return json.load(f)

    @staticmethod
    def write_json(path: str, data: Dict[str, Any], indent: int = 2):
        """
        Write data to JSON file.

        Args:
            path: Path to write
            data: Data to write
            indent: JSON indentation
        """
        with open(path, 'w') as f:
            json.dump(data, f, indent=indent)

    @staticmethod
    def read_yaml(path: str) -> Dict[str, Any]:
        """
        Read and parse YAML file.

        Args:
            path: Path to YAML file

        Returns:
            Parsed YAML data
        """
        import yaml
        with open(path, 'r') as f:
            return yaml.safe_load(f)

    @staticmethod
    def write_yaml(path: str, data: Dict[str, Any]):
        """
        Write data to YAML file.

        Args:
            path: Path to write
            data: Data to write
        """
        import yaml
        with open(path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False)

    @staticmethod
    def modify_yaml(path: str, modifications: Dict[str, Any]):
        """
        Modify YAML file with given modifications.

        Args:
            path: Path to YAML file
            modifications: Dict of key-value pairs to update
        """
        import yaml
        with open(path, 'r') as f:
            data = yaml.safe_load(f) or {}

        data.update(modifications)

        with open(path, 'w') as f:
            yaml.dump(data, f, default_flow_style=False)

    @staticmethod
    def file_contains(path: str, text: str) -> bool:
        """
        Check if file contains specific text.

        Args:
            path: Path to file
            text: Text to search for

        Returns:
            True if text found in file
        """
        try:
            with open(path, 'r') as f:
                content = f.read()
            return text in content
        except (IOError, OSError):
            return False

    @staticmethod
    def file_exists(path: str) -> bool:
        """
        Check if file exists.

        Args:
            path: Path to check

        Returns:
            True if file exists
        """
        return os.path.isfile(path)

    @staticmethod
    def dir_exists(path: str) -> bool:
        """
        Check if directory exists.

        Args:
            path: Path to check

        Returns:
            True if directory exists
        """
        return os.path.isdir(path)

    @staticmethod
    def create_temp_dir(prefix: str = "isolde-e2e-") -> str:
        """
        Create temporary directory.

        Args:
            prefix: Directory name prefix

        Returns:
            Path to created directory
        """
        import tempfile
        return tempfile.mkdtemp(prefix=prefix)

    @staticmethod
    def remove_dir(path: str):
        """
        Remove directory and all contents.

        Args:
            path: Path to remove
        """
        import shutil
        shutil.rmtree(path, ignore_errors=True)


class ValidationHelper:
    """Helper class for validation operations."""

    @staticmethod
    def is_valid_json(text: str) -> bool:
        """
        Check if text is valid JSON.

        Args:
            text: Text to validate

        Returns:
            True if valid JSON
        """
        try:
            json.loads(text)
            return True
        except json.JSONDecodeError:
            return False

    @staticmethod
    def is_valid_yaml(text: str) -> bool:
        """
        Check if text is valid YAML.

        Args:
            text: Text to validate

        Returns:
            True if valid YAML
        """
        try:
            import yaml
            yaml.safe_load(text)
            return True
        except yaml.YAMLError:
            return False

    @staticmethod
    def extract_json_output(text: str) -> Optional[Dict[str, Any]]:
        """
        Extract JSON from text that may contain other content.

        Args:
            text: Text that may contain JSON

        Returns:
            Parsed JSON or None if not found
        """
        # Try parsing entire text as JSON first
        if ValidationHelper.is_valid_json(text):
            return json.loads(text)

        # Try to find JSON object in text
        start = text.find('{')
        if start == -1:
            start = text.find('[')

        if start != -1:
            # Try to find matching bracket
            bracket_count = 0
            in_string = False
            escape = False
            end = start

            for i in range(start, len(text)):
                char = text[i]

                if escape:
                    escape = False
                    continue

                if char == '\\':
                    escape = True
                    continue

                if char == '"' and not escape:
                    in_string = not in_string
                    continue

                if not in_string:
                    if char in '{[(':
                        bracket_count += 1
                    elif char in '})]':
                        bracket_count -= 1
                        if bracket_count == 0:
                            end = i + 1
                            break

            json_text = text[start:end]
            if ValidationHelper.is_valid_json(json_text):
                return json.loads(json_text)

        return None

    @staticmethod
    def check_command_exists(command: str) -> bool:
        """
        Check if command is available in PATH.

        Args:
            command: Command to check

        Returns:
            True if command exists
        """
        result = subprocess.run(
            ["which", command],
            capture_output=True,
            text=True
        )
        return result.returncode == 0


class IsoldeHelper:
    """Helper class for Isolde CLI operations."""

    @staticmethod
    def run_isolde_command(args: List[str], cwd: Optional[str] = None,
                          timeout: int = 60) -> subprocess.CompletedProcess:
        """
        Run isolde command.

        Args:
            args: Command arguments (without 'isolde')
            cwd: Working directory
            timeout: Command timeout

        Returns:
            CompletedProcess result
        """
        cmd = ["isolde"] + args
        return subprocess.run(
            cmd,
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=timeout
        )

    @staticmethod
    def get_isolde_version() -> Optional[str]:
        """
        Get isolde version.

        Returns:
            Version string or None
        """
        result = IsoldeHelper.run_isolde_command(["--version"])
        if result.returncode == 0:
            return result.stdout.strip().split()[-1]
        return None

    @staticmethod
    def list_templates() -> List[str]:
        """
        Get list of available templates.

        Returns:
            List of template names
        """
        result = IsoldeHelper.run_isolde_command(["list-templates"])
        if result.returncode == 0:
            return [line.strip() for line in result.stdout.split('\n') if line.strip()]
        return []

    @staticmethod
    def list_presets() -> List[str]:
        """
        Get list of available presets.

        Returns:
            List of preset names
        """
        result = IsoldeHelper.run_isolde_command(["list-presets"])
        if result.returncode == 0:
            return [line.strip() for line in result.stdout.split('\n') if line.strip()]
        return []


class ProjectHelper:
    """Helper class for project operations."""

    @staticmethod
    def get_project_path(workspace: str, project_name: str) -> str:
        """
        Get full path to project.

        Args:
            workspace: Workspace directory
            project_name: Project name

        Returns:
            Full path to project
        """
        return os.path.join(workspace, project_name)

    @staticmethod
    def get_devcontainer_path(project_path: str) -> str:
        """
        Get path to .devcontainer directory.

        Args:
            project_path: Path to project

        Returns:
            Path to .devcontainer directory
        """
        return os.path.join(project_path, ".devcontainer")

    @staticmethod
    def get_isolde_yaml_path(project_path: str) -> str:
        """
        Get path to isolde.yaml.

        Args:
            project_path: Path to project

        Returns:
            Path to isolde.yaml
        """
        return os.path.join(project_path, "isolde.yaml")

    @staticmethod
    def project_has_git(project_path: str) -> bool:
        """
        Check if project is a git repository.

        Args:
            project_path: Path to project

        Returns:
            True if .git directory exists
        """
        git_dir = os.path.join(project_path, ".git")
        return os.path.isdir(git_dir)

    @staticmethod
    def get_project_files(project_path: str, relative: bool = True) -> List[str]:
        """
        Get list of files in project.

        Args:
            project_path: Path to project
            relative: Return relative paths

        Returns:
            List of file paths
        """
        files = []
        for root, _, filenames in os.walk(project_path):
            # Skip .git directory
            if '.git' in root.split(os.sep):
                continue

            for filename in filenames:
                full_path = os.path.join(root, filename)
                if relative:
                    rel_path = os.path.relpath(full_path, project_path)
                    files.append(rel_path)
                else:
                    files.append(full_path)

        return files


def cleanup_test_resources(project_name: Optional[str] = None,
                          workspace: Optional[str] = None):
    """
    Clean up test resources.

    Args:
        project_name: Optional project name for container cleanup
        workspace: Optional workspace directory to remove
    """
    # Stop and remove containers
    if project_name:
        ContainerHelper.remove_container(project_name, force=True)

    # Remove workspace
    if workspace and os.path.exists(workspace):
        FileHelper.remove_dir(workspace)


def wait_for_condition(condition_func, timeout: int = 30,
                      interval: float = 0.5) -> bool:
    """
    Wait for a condition to become true.

    Args:
        condition_func: Function that returns bool
        timeout: Maximum wait time in seconds
        interval: Check interval in seconds

    Returns:
        True if condition became true
    """
    start = time.time()
    while time.time() - start < timeout:
        if condition_func():
            return True
        time.sleep(interval)
    return False
