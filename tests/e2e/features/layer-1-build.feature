@slow @priority @layer-1
Feature: Template Build Matrix
  As a developer
  I want all templates to build successfully with all Claude plugins
  So that I can trust the template system for any development environment

  Scenario Outline: Build template with all Claude plugins
    Given I am using the "shell-script" generator
    When I create a project named "test-<template>" using template "<template>" with version "<version>"
    And I activate all Claude plugins
    Then the project should be created successfully
    And the devcontainer should build successfully
    And all core tools should be available for "<template>"
    And Claude Code CLI should be installed
    And oh-my-claudecode plugin should be activated
    And the devcontainer features should be valid

    Examples: Templates with default versions
      | template | version |
      | python   | 3.12    |
      | nodejs   | 22      |
      | rust     | latest  |
      | go       | latest  |
      | generic  | latest  |

  Scenario Outline: Build template with specific Claude providers
    Given I am using the "shell-script" generator
    When I create a project named "test-<template>-<provider>" using template "<template>" with Claude provider "<provider>"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Claude Code CLI should be configured for provider "<provider>"

    Examples: Provider combinations
      | template | provider    |
      | python   | anthropic  |
      | python   | openai     |
      | nodejs   | anthropic  |
      | nodejs   | bedrock    |
      | rust     | anthropic  |
      | rust     | vertex     |

  Scenario Outline: Build template with plugin combinations
    Given I am using the "shell-script" generator
    When I create a project named "test-<template>-<plugin>" using template "<template>"
    And I activate Claude plugin "<plugin>"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And plugin "<plugin>" should be available in the container

    Examples: Plugin activation
      | template | plugin              |
      | python   | oh-my-claudecode    |
      | python   | tdd                 |
      | nodejs   | oh-my-claudecode    |
      | nodejs   | security-review     |
      | rust     | oh-my-claudecode    |
      | rust     | code-review         |
      | go       | oh-my-claudecode    |
