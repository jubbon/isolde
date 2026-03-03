# -*- coding: utf-8 -*-
Feature: Container Management Commands
  Test complete container lifecycle through Isolde CLI

  @container @priority
  Scenario: Complete container lifecycle
    Given I am using the "shell-script" generator
    When I create a project named "test-lifecycle" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build"
    Then isolde build should succeed
    When I run "isolde run --detach"
    Then the container should be running
    When I run "isolde exec python --version"
    Then the output should contain "Python"
    When I run "isolde ps"
    Then I should see the container in the list
    When I run "isolde logs --tail 10"
    Then logs should be displayed
    When I run "isolde stop"
    Then the container should be stopped
    When I run "isolde ps --all"
    Then I should see the stopped container

  @container
  Scenario: Build with options
    Given I am using the "shell-script" generator
    When I create a project named "test-build-options" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build --no-cache"
    Then isolde build should succeed
    And the build should not use layer cache
    When I run "isolde build --image-name test-custom:v1.0"
    Then isolde build should succeed
    And the image should be tagged "test-custom:v1.0"

  @container
  Scenario: Exec multiple commands
    Given I am using the "shell-script" generator
    When I create a project named "test-exec" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build"
    Then isolde build should succeed
    When I run "isolde run --detach"
    Then the container should be running
    When I run "isolde exec python -m pytest --version"
    Then pytest should be available
    When I run "isolde exec bash -c 'echo hello && pwd'"
    Then both commands should execute

  @container
  Scenario: Build with workspace folder option
    Given I am using the "shell-script" generator
    When I create a project named "test-workspace" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build --workspace-folder ."
    Then isolde build should succeed

  @container
  Scenario: Run and exec in detached mode
    Given I am using the "shell-script" generator
    When I create a project named "test-detach" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build"
    Then isolde build should succeed
    When I run "isolde run --detach --workspace-folder ."
    Then the container should be running
    And the container should be running in background

  @container
  Scenario: Logs with follow option
    Given I am using the "shell-script" generator
    When I create a project named "test-logs" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build"
    Then isolde build should succeed
    When I run "isolde run --detach"
    Then the container should be running
    When I run "isolde exec bash -c 'echo test log message'"
    When I run "isolde logs --tail 5"
    Then logs should be displayed
    And logs should contain "test log message"

  @container
  Scenario: Stop with force option
    Given I am using the "shell-script" generator
    When I create a project named "test-force-stop" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde build"
    Then isolde build should succeed
    When I run "isolde run --detach"
    Then the container should be running
    When I run "isolde stop --force"
    Then the container should be stopped

  @container @error
  Scenario: Build without .devcontainer directory
    Given I have a project without .devcontainer
    When I run "isolde build"
    Then the command should fail with error containing "devcontainer"

  @container @error
  Scenario: Exec with no running container
    Given I have a synced project without running container
    When I run "isolde exec echo test"
    Then the command should fail with error containing "container"

  @container @error
  Scenario: Stop with no running container
    Given I have a synced project without running container
    When I run "isolde stop"
    Then the command should fail or succeed with no container

  @container @error
  Scenario: Logs with no container
    Given I have a synced project without running container
    When I run "isolde logs"
    Then the command should fail with error containing "container"
