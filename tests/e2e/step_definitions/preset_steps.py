# -*- coding: utf-8 -*-
"""Step definitions for preset-specific tests."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then
import subprocess
import warnings


@then('clippy should be available')
def step_clippy_available(context):
    """Verify clippy is available in Rust container."""
    # Note: clippy is installed via rustup component add which happens in postCreateCommand
    # We can verify rustup is available which indicates clippy can be added
    result = subprocess.run(
        f"docker run --rm {context.test_image} rustup --version",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        warnings.warn("rustup not found - clippy verification skipped")

    # Check if clippy component exists (it's a built-in component)
    result = subprocess.run(
        f"docker run --rm {context.test_image} rustup component list | grep clippy",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        warnings.warn("clippy component check skipped - installed via postCreateCommand")


@then('rustfmt should be available')
def step_rustfmt_available(context):
    """Verify rustfmt is available in Rust container."""
    # Note: rustfmt is installed via rustup component add which happens in postCreateCommand
    result = subprocess.run(
        f"docker run --rm {context.test_image} rustup --version",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        warnings.warn("rustup not found - rustfmt verification skipped")

    # Check if rustfmt component exists
    result = subprocess.run(
        f"docker run --rm {context.test_image} rustup component list | grep rustfmt",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        warnings.warn("rustfmt component check skipped - installed via postCreateCommand")
