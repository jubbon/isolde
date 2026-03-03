# -*- coding: utf-8 -*-
Feature: Error Scenarios
  Test error handling and validation for Isolde CLI commands

  @error @init
  Scenario: Invalid template name
    Given I am using the "shell-script" generator
    When I run error test isolde init template-only "test-invalid" using "nonexistent-template-xyz"
    Then the command should fail with error containing "template"

  @error @init
  Scenario: Invalid preset name
    Given I am using the "shell-script" generator
    When I run error test isolde init preset-only "test-invalid" using "nonexistent-preset-xyz"
    Then the command should fail with error containing "preset"

  @error @init
  Scenario: Project directory already exists
    Given I am using the "shell-script" generator
    When I create a project named "test-existing" using template "python"
    Then the project should be created successfully
    When I run "isolde init test-existing --template python"
    Then the command should fail or ask for confirmation

  @error @sync
  Scenario: Sync without isolde.yaml
    Given I have a directory without isolde.yaml
    When I run "isolde sync"
    Then the command should fail with error containing "isolde" or "yaml"

  @error @build
  Scenario: Build without .devcontainer
    Given I have a project without .devcontainer
    When I run "isolde build"
    Then the command should fail with error containing "devcontainer"

  @error @build
  Scenario: Build with invalid devcontainer.json
    Given I have a project with invalid devcontainer.json
    When I run "isolde build"
    Then the command should fail with error containing "json" or "parse"

  @error @run
  Scenario: Run with no image built
    Given I have a synced project without building image
    When I run "isolde run --detach"
    Then the command should fail with error containing "image" or "build"

  @error @exec
  Scenario: Exec with no running container
    Given I have a synced project without running container
    When I run "isolde exec echo test"
    Then the command should fail with error containing "container"

  @error @exec
  Scenario: Exec with invalid command
    Given I have a synced project without running container
    When I run "isolde exec nonexistent-command-xyz"
    Then the command should fail with error containing "command" or "not found"

  @error @stop
  Scenario: Stop with no container
    Given I have a synced project without running container
    When I run "isolde stop"
    Then the command should fail or succeed with no container

  @error @logs
  Scenario: Logs with no container
    Given I have a synced project without running container
    When I run "isolde logs"
    Then the command should fail with error containing "container"

  @error @validate
  Scenario: Invalid isolde.yaml
    Given I have a project with malformed isolde.yaml
    When I run "isolde validate"
    Then validation should fail with error containing "yaml" or "parse"

  @error @validate
  Scenario: Invalid Claude provider
    Given I have a project with invalid Claude provider
    When I run "isolde validate"
    Then validation should fail with error containing "provider"

  @error @validate
  Scenario: Missing required fields in isolde.yaml
    Given I have a project with incomplete isolde.yaml
    When I run "isolde validate"
    Then validation should fail with error containing "required" or "missing"

  @error @init
  Scenario: Invalid language version
    Given I am using the "shell-script" generator
    When I run error test isolde init with-version "test-invalid" template "python" version "99.99"
    Then the command should fail with error containing "version"

  @error @init
  Scenario: Missing project name
    Given I am using the "shell-script" generator
    When I run error test isolde init missing-name template "python"
    Then the command should fail or prompt for project name

  @error @diff
  Scenario: Diff without template reference
    Given I have a synced project
    And I remove the template reference
    When I run "isolde diff"
    Then the command should fail with error containing "template" or "reference"

  @error @doctor
  Scenario: Doctor with missing Docker
    Given Docker is not available on the system
    When I run "isolde doctor --component docker"
    Then the check should fail with error containing "docker" or "not found"

  @error @sync
  Scenario: Sync with conflicting configuration
    Given I have a project with conflicting isolde.yaml
    When I run "isolde sync --force"
    Then the command should fail or overwrite with warnings

  @error @init
  Scenario: Invalid proxy configuration
    Given I am using the "shell-script" generator
    When I run error test isolde init proxy "test-proxy" template "python" http-proxy "invalid-url"
    Then the command should fail with error containing "proxy" or "url"

  @error @build
  Scenario: Build with Docker daemon not running
    Given Docker daemon is stopped
    And I have a synced project
    When I run "isolde build"
    Then the command should fail with error containing "docker" or "daemon"

  @error @run
  Scenario: Run with invalid workspace folder
    Given I have a synced project
    When I run "isolde build"
    And I run "isolde run --workspace-folder /nonexistent/folder"
    Then the command should fail with error containing "folder" or "not found"

  @error @ps
  Scenario: PS with invalid project context
    Given I have a directory without devcontainer
    When I run "isolde ps"
    Then the command should fail or show empty list

  @error @init
  Scenario: Both template and preset specified
    Given I am using the "shell-script" generator
    When I run error test isolde init both "test-both" template "python" preset "python-ml"
    Then the command should fail with error containing "template" and "preset"
