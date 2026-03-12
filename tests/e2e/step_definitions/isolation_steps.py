"""Step definitions for isolation level testing."""

import json
import os
import subprocess

from behave import when, then


@when('I create a project with isolation level "{level}"')
def step_create_with_isolation(context, level):
    """Create a project and set the isolation level in isolde.yaml, then re-sync."""
    context.project_name = f"test-isolation-{level}"

    # Generate project using the standard generator
    result = context.generator.generate(
        context.test_workspace,
        project_name=context.project_name,
        template="python",
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout

    if result.returncode != 0:
        return

    project_path = os.path.join(context.test_workspace, context.project_name)
    isolde_yaml = os.path.join(project_path, "isolde.yaml")

    # Read and modify isolde.yaml to set isolation level
    with open(isolde_yaml, "r") as f:
        content = f.read()

    # Add or replace isolation field
    if "isolation:" in content:
        lines = content.splitlines()
        new_lines = []
        for line in lines:
            if line.strip().startswith("isolation:"):
                new_lines.append(f"isolation: {level}")
            else:
                new_lines.append(line)
        content = "\n".join(new_lines) + "\n"
    else:
        content += f"isolation: {level}\n"

    with open(isolde_yaml, "w") as f:
        f.write(content)

    # Re-run isolde sync to regenerate devcontainer.json with new isolation
    repo_root = os.path.dirname(
        os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    )
    isolde_bin = os.path.join(repo_root, "scripts", "isolde.sh")

    result = subprocess.run(
        [isolde_bin, "sync", "--force"],
        cwd=project_path,
        capture_output=True,
        text=True,
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout

    # Store devcontainer path for later assertions
    context.devcontainer_json_path = os.path.join(
        project_path, ".devcontainer", "devcontainer.json"
    )


@then("devcontainer.json should have {count:d} mounts")
def step_check_mount_count(context, count):
    """Verify the number of mounts in devcontainer.json."""
    with open(context.devcontainer_json_path, "r") as f:
        config = json.load(f)

    mounts = config.get("mounts", [])
    assert len(mounts) == count, (
        f"Expected {count} mounts, got {len(mounts)}:\n"
        + "\n".join(f"  - {m}" for m in mounts)
    )


@then('a mount should reference "{text}"')
def step_mount_contains(context, text):
    """Verify at least one mount contains the given text."""
    with open(context.devcontainer_json_path, "r") as f:
        config = json.load(f)

    mounts = config.get("mounts", [])
    assert any(text in m for m in mounts), (
        f"No mount references '{text}':\n"
        + "\n".join(f"  - {m}" for m in mounts)
    )


@then('no mount should reference "{text}"')
def step_mount_not_contains(context, text):
    """Verify no mount contains the given text."""
    with open(context.devcontainer_json_path, "r") as f:
        config = json.load(f)

    mounts = config.get("mounts", [])
    matches = [m for m in mounts if text in m]
    assert not matches, (
        f"Mount(s) unexpectedly reference '{text}':\n"
        + "\n".join(f"  - {m}" for m in matches)
    )
