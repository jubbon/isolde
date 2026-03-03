"""Generator abstraction - agnostic to shell-script or Rust binary."""

import subprocess
from typing import Dict
import os
import shlex


class GeneratorInterface:
    """Base interface for project generators."""

    def generate(self, name: str, **kwargs) -> subprocess.CompletedProcess:
        """Generate a project."""
        raise NotImplementedError


class ShellScriptGenerator(GeneratorInterface):
    """Shell script generator implementation."""

    def generate(self, name: str, workspace: str = None, **kwargs) -> subprocess.CompletedProcess:
        # Change to repo root first
        repo_root = self._get_repo_root()

        # Quote the project name to handle spaces and special characters
        quoted_name = shlex.quote(name)

        # The new CLI creates projects in the current directory, not a subdirectory
        # We need to create the subdirectory first and cd into it
        if workspace:
            project_path = os.path.join(workspace, name)
            cmd = f"mkdir -p {project_path} && cd {project_path} && {repo_root}/scripts/isolde.sh init ."
        else:
            project_path = os.path.join(repo_root, name)
            cmd = f"mkdir -p {project_path} && cd {project_path} && {repo_root}/scripts/isolde.sh init ."

        if 'template' in kwargs:
            cmd += f" --template={kwargs['template']}"
        if 'lang_version' in kwargs:
            cmd += f" --lang-version={kwargs['lang_version']}"
        if 'preset' in kwargs:
            cmd += f" --preset={kwargs['preset']}"
        if 'http_proxy' in kwargs:
            cmd += f" --http-proxy={kwargs['http_proxy']}"
        if 'https_proxy' in kwargs:
            cmd += f" --https-proxy={kwargs['https_proxy']}"
        if 'claude_provider' in kwargs:
            cmd += f" --claude-provider={kwargs['claude_provider']}"
        if 'claude_version' in kwargs:
            cmd += f" --claude-version={kwargs['claude_version']}"

        # The new CLI requires sync after init
        # Add sync command to the same shell call
        cmd += f" && {repo_root}/scripts/isolde.sh sync"

        # Initialize git repository (the new CLI doesn't do this automatically)
        cmd += f" && git init && git add -A && git commit -m 'Initial commit'"

        # Pipe newlines to accept all defaults for non-interactive mode
        # Each prompt in the script will get a newline (accepting default value)
        input_data = "\n\n\n\n\n\n\n\n\n\n"

        return subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            input=input_data
        )

    def _get_repo_root(self):
        """Get the repository root directory."""
        # File is at tests/e2e/support/generators.py
        # Need to go up 4 levels to reach repo root
        return os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))


class ClaudeBinaryGenerator(GeneratorInterface):
    """Rust binary generator implementation (future)."""

    def generate(self, name: str, **kwargs) -> subprocess.CompletedProcess:
        cmd = f"claude init {name}"
        return subprocess.run(cmd, shell=True, capture_output=True, text=True)


def get_generator(generator_type: str = "shell-script") -> GeneratorInterface:
    """Factory function to get generator instance."""
    generators = {
        "shell-script": ShellScriptGenerator,
        "claude-binary": ClaudeBinaryGenerator,
    }
    return generators.get(generator_type, ShellScriptGenerator)()
