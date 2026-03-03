# -*- coding: utf-8 -*-
# Feature: In-Container Verification Grid

@fast @layer-3
Feature: In-Container Verification Grid
  As a developer
  I want basic verification checks to pass
  So that I have confidence in the development environment quality

  Scenario: Verify Python template structure
    Given I am using the "shell-script" generator
    When I create a project named "test-python" using template "python"
    Then the project should be created successfully
    And the project should be a git repository
    And the devcontainer directory should exist
    And devcontainer.json should exist

  Scenario: Verify Node.js template structure
    Given I am using the "shell-script" generator
    When I create a project named "test-nodejs" using template "nodejs"
    Then the project should be created successfully
    And the project should be a git repository
    And the devcontainer directory should exist
    And devcontainer.json should exist

  Scenario: Verify Rust template structure
    Given I am using the "shell-script" generator
    When I create a project named "test-rust" using template "rust"
    Then the project should be created successfully
    And the project should be a git repository
    And the devcontainer directory should exist
    And devcontainer.json should exist

  Scenario: Verify Go template structure
    Given I am using the "shell-script" generator
    When I create a project named "test-go" using template "go"
    Then the project should be created successfully
    And the project should be a git repository
    And the devcontainer directory should exist
    And devcontainer.json should exist

  Scenario: Verify Generic template structure
    Given I am using the "shell-script" generator
    When I create a project named "test-generic" using template "generic"
    Then the project should be created successfully
    And the project should be a git repository
    And the devcontainer directory should exist
    And devcontainer.json should exist
