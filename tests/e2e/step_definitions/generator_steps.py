# -*- coding: utf-8 -*-
"""Step definitions for project generation."""

from behave import given, when
import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))
from generators import get_generator


@given('I am using the "{generator_type}" generator')
def step_use_generator(context, generator_type):
    """Specify which generator to use."""
    context.generator = get_generator(generator_type)
    context.generator_type = generator_type


@when('I create a project named "{name}" using template "{template}" with preset "{preset}"')
def step_create_project_with_preset(context, name, template, preset):
    """Create a project using specified template and preset."""
    context.project_name = name
    context.template = template
    context.preset = preset

    result = context.generator.generate(name, workspace=context.test_workspace, template=template, preset=preset)
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@given('I create a project named "{name}" using template "{template}"')
def step_create_project(context, name, template):
    """Create a project using specified template."""
    context.project_name = name
    context.template = template

    result = context.generator.generate(name, workspace=context.test_workspace, template=template)
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I specify language version "{version}"')
def step_set_version(context, version):
    """Set language version and regenerate project."""
    context.language_version = version
    # Re-generate with version
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template=context.template,
        lang_version=version
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I use preset "{preset}"')
def step_use_preset(context, preset):
    """Use a specific preset."""
    context.preset = preset
    # Re-generate with preset
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template=context.template,
        preset=preset
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I create a project named "{name}" using template "{template}"')
def step_when_create_project(context, name, template):
    """Create a project using specified template."""
    context.project_name = name
    context.template = template

    result = context.generator.generate(name, workspace=context.test_workspace, template=template)
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr
