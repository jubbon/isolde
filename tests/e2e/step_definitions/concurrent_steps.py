# -*- coding: utf-8 -*-
"""Step definitions for concurrent operation tests."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import when, then, given
import subprocess
import threading
import time


@when('I create projects named {names} using template "{template}" simultaneously')
def step_create_concurrent(context, names, template):
    """Create multiple projects concurrently."""
    # Parse names from comma-separated list
    name_list = [n.strip().strip('"') for n in names.split(',')]
    context.concurrent_projects = name_list

    threads = []
    results = {}
    lock = threading.Lock()

    def create_project(name):
        result = context.generator.generate(
            name,
            workspace=context.test_workspace,
            template=template
        )
        with lock:
            results[name] = result

    for name in name_list:
        thread = threading.Thread(target=create_project, args=(name,))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

    context.concurrent_results = results


@then('all projects should be created successfully')
def step_all_created(context):
    """Verify all concurrent creations succeeded."""
    for name, result in context.concurrent_results.items():
        assert result.returncode == 0, f"Project {name} creation failed: {result.stderr}"
        path = os.path.join(context.test_workspace, name)
        assert os.path.exists(path), f"Project {name} path not found at {path}"


@then('each project should have independent structure')
def step_independent_structure(context):
    """Verify each project has its own independent structure."""
    for name in context.concurrent_projects:
        project_path = os.path.join(context.test_workspace, name)

        # Check basic structure exists
        assert os.path.isdir(project_path), f"Project directory {name} not found"

        devcontainer_path = os.path.join(project_path, ".devcontainer")
        assert os.path.isdir(devcontainer_path), f".devcontainer not found in {name}"

        devcontainer_json = os.path.join(devcontainer_path, "devcontainer.json")
        assert os.path.isfile(devcontainer_json), f"devcontainer.json not found in {name}"


@when('I create "{name1}" using template "{template1}" and "{name2}" using template "{template2}" simultaneously')
def step_create_different_concurrent(context, name1, template1, name2, template2):
    """Create different templates concurrently."""
    context.concurrent_projects = [name1, name2]

    results = {}
    lock = threading.Lock()

    def create_project(name, template):
        result = context.generator.generate(
            name,
            workspace=context.test_workspace,
            template=template
        )
        with lock:
            results[name] = result

    t1 = threading.Thread(target=create_project, args=(name1, template1))
    t2 = threading.Thread(target=create_project, args=(name2, template2))

    t1.start()
    t2.start()

    t1.join()
    t2.join()

    context.concurrent_results = results


@then('both projects should be created successfully')
def step_both_created(context):
    """Verify both concurrent creations succeeded."""
    step_all_created(context)
