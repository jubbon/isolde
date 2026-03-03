@medium @layer-2
Feature: User Scenarios from isolde.yaml
  As a developer
  I want real-world user workflows to work correctly
  So that I can use Isolde for actual development projects

  Scenario: Python basic workflow
    Given I am using the "shell-script" generator
    When I create a project named "python-basic" using template "python" with version "3.12"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Python 3.12 should be installed in the container
    And uv should be available in the container
    And pytest should be available in the container
    And I can run a simple Python script

  Scenario: Python ML workflow
    Given I am using the "shell-script" generator
    When I create a project named "python-ml" using template "python" with preset "python-ml"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Jupyter should be installed in the container
    And numpy should be importable
    And pandas should be importable
    And I can create a simple Jupyter notebook

  Scenario: Python web workflow
    Given I am using the "shell-script" generator
    When I create a project named "python-web" using template "python" with preset "python-web"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And pytest should be available in the container
    And ruff should be available for linting
    And I can create a simple Flask application

  Scenario: Node.js API workflow
    Given I am using the "shell-script" generator
    When I create a project named "node-api" using template "nodejs" with preset "node-api"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Node.js 22 should be installed in the container
    And TypeScript should be installed
    And ESLint should be configured
    And vitest should be available
    And I can create a simple Express API

  Scenario: Rust CLI workflow
    Given I am using the "shell-script" generator
    When I create a project named "rust-cli" using template "rust" with preset "rust-cli"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Rust should be installed in the container
    And cargo should be available in the container
    And clippy should be available
    And rustfmt should be available
    And I can build a simple Rust binary

  Scenario: Go service workflow
    Given I am using the "shell-script" generator
    When I create a project named "go-service" using template "go" with preset "go-service"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Go should be installed in the container
    And golangci-lint should be available
    And I can create a simple Go module

  Scenario: Fullstack workflow
    Given I am using the "shell-script" generator
    When I create a project named "fullstack-app" using template "nodejs" with preset "fullstack"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And TypeScript should be installed
    And ESLint should be configured
    And Prettier should be configured
    And vitest should be available
    And I can create a simple fullstack project

  Scenario: Minimal workflow
    Given I am using the "shell-script" generator
    When I create a project named "minimal-project" using template "generic" with preset "minimal"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And the container should have basic tools
    And I can run basic shell commands

  Scenario: Multi-language project workflow
    Given I am using the "shell-script" generator
    When I create a project named "multi-lang" using template "python" with version "3.12"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And I can add Node.js to the project
    And I can run both Python and Node.js scripts

  Scenario: Custom isolde.yaml workflow
    Given I have a custom isolde.yaml configuration
    When I create a project using the custom configuration
    Then the project should be created successfully
    And the devcontainer should build successfully
    And all custom settings should be applied
    And the custom provider should be configured
