"""Generator abstraction - agnostic to shell-script or Rust binary."""

import subprocess
from typing import Dict
import os


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

        # If workspace provided, cd there first (for testing)
        if workspace:
            cmd = f"cd {workspace} && {repo_root}/scripts/init-project.sh {name}"
        else:
            cmd = f"cd {repo_root} && ./scripts/init-project.sh {name}"

        if 'template' in kwargs:
            cmd += f" --template={kwargs['template']}"
        if 'lang_version' in kwargs:
            cmd += f" --lang-version={kwargs['lang_version']}"
        if 'preset' in kwargs:
            cmd += f" --preset={kwargs['preset']}"

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
