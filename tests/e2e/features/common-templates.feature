Feature: Common Template Requirements
  As a developer
  I want all devcontainers to meet standards

  Scenario: Rust template creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-rust" using template "rust"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Rust should be installed in the container
    And cargo should be available in the container

  Scenario: Go template creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-go" using template "go"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Go should be installed in the container
    And golangci-lint should be available
