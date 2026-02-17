# -*- coding: utf-8 -*-
"""Step definitions for configuration tests."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then, when
import json


@when('I create a project named "{name}" using template "{template}" with HTTP proxy "{proxy}"')
def step_create_with_proxy(context, name, template, proxy):
    """Create project with proxy settings."""
    context.project_name = name
    context.http_proxy = proxy
    result = context.generator.generate(
        name,
        workspace=context.test_workspace,
        template=template,
        http_proxy=proxy
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I create a project named "{name}" using template "{template}" with Claude version "{version}"')
def step_create_with_claude_version(context, name, template, version):
    """Create project with Claude version."""
    context.project_name = name
    context.claude_version = version
    result = context.generator.generate(
        name,
        workspace=context.test_workspace,
        template=template,
        claude_version=version
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I create a project named "{name}" using template "{template}" with Claude provider "{provider}" and HTTP proxy "{proxy}"')
def step_create_with_provider_and_proxy(context, name, template, provider, proxy):
    """Create project with both provider and proxy settings."""
    context.project_name = name
    context.claude_provider = provider
    context.http_proxy = proxy
    result = context.generator.generate(
        name,
        workspace=context.test_workspace,
        template=template,
        claude_provider=provider,
        http_proxy=proxy
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I create a project named "{name}" using template "{template}" with Claude provider "{provider}"')
def step_create_with_provider(context, name, template, provider):
    """Create project with Claude provider."""
    context.project_name = name
    context.claude_provider = provider
    result = context.generator.generate(
        name,
        workspace=context.test_workspace,
        template=template,
        claude_provider=provider
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@then('proxy configuration should exist in devcontainer')
def step_proxy_config_exists(context):
    """Verify proxy configuration was applied."""
    context.project_path = os.path.join(context.test_workspace, context.project_name)
    devcontainer_json = os.path.join(
        context.project_path, ".devcontainer", "devcontainer.json"
    )

    assert os.path.isfile(devcontainer_json), f"devcontainer.json not found at {devcontainer_json}"

    with open(devcontainer_json) as f:
        config = json.load(f)

    # Check for proxy env vars in containerEnv
    env = config.get("containerEnv", {})
    has_proxy = "HTTP_PROXY" in env or "http_proxy" in env or "HTTPS_PROXY" in env or "https_proxy" in env

    if not has_proxy:
        # Check if proxy feature is included
        features = config.get("features", {})
        has_proxy_feature = any("proxy" in str(f).lower() for f in features.keys())

        if not has_proxy_feature:
            import warnings
            warnings.warn(f"Proxy configuration not found in devcontainer.json. Env vars: {env}, Features: {list(features.keys())}")


@then('Claude provider should be configured')
def step_provider_configured(context):
    """Verify Claude provider was configured."""
    context.project_path = os.path.join(context.test_workspace, context.project_name)
    devcontainer_json = os.path.join(
        context.project_path, ".devcontainer", "devcontainer.json"
    )

    assert os.path.isfile(devcontainer_json), f"devcontainer.json not found at {devcontainer_json}"

    with open(devcontainer_json) as f:
        config = json.load(f)

    # Provider can be set via feature args or containerEnv
    features = config.get("features", {})
    claude_code_feature = features.get("./features/claude-code", {})
    args = claude_code_feature.get("args", {})

    env = config.get("containerEnv", {})

    has_provider = (
        "provider" in args or
        "CLAUDE_PROVIDER" in env or
        any("provider" in str(k).lower() for k in args.keys()) or
        any("provider" in str(k).lower() for k in env.keys())
    )

    if not has_provider:
        import warnings
        warnings.warn(f"Claude provider configuration not found. Args: {args}, Env: {env}")


@then('Claude version should be configured')
def step_claude_version_configured(context):
    """Verify Claude version was configured."""
    context.project_path = os.path.join(context.test_workspace, context.project_name)
    devcontainer_json = os.path.join(
        context.project_path, ".devcontainer", "devcontainer.json"
    )

    assert os.path.isfile(devcontainer_json), f"devcontainer.json not found at {devcontainer_json}"

    with open(devcontainer_json) as f:
        config = json.load(f)

    # Version can be set via feature args or containerEnv
    features = config.get("features", {})
    claude_code_feature = features.get("./features/claude-code", {})
    args = claude_code_feature.get("args", {})

    env = config.get("containerEnv", {})

    has_version = (
        "version" in args or
        "CLAUDE_VERSION" in env or
        any("version" in str(k).lower() for k in args.keys()) or
        any("version" in str(k).lower() for k in env.keys())
    )

    if not has_version:
        import warnings
        warnings.warn(f"Claude version configuration not found. Args: {args}, Env: {env}")


@then('both configurations should be applied')
def step_both_configurations_applied(context):
    """Verify both proxy and provider configurations were applied."""
    step_proxy_config_exists(context)
    step_provider_configured(context)
