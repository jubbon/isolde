Feature: Preset Coverage
  As a developer
  I want all presets to work correctly
  So that I can quickly create projects with pre-configured settings

  @preset
  Scenario: rust-cli preset creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-rust-cli" using template "rust" with preset "rust-cli"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Rust should be installed in the container
    And cargo should be available in the container
    And clippy should be available
    And rustfmt should be available

  @preset
  Scenario: go-service preset creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-go-service" using template "go" with preset "go-service"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Go should be installed in the container
    And golangci-lint should be available

  @preset
  Scenario: minimal preset creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-minimal" using template "generic" with preset "minimal"
    Then the project should be created successfully
    And the devcontainer should build successfully

  @preset
  Scenario: fullstack preset creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-fullstack" using template "nodejs" with version "22" and preset "fullstack"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And TypeScript should be installed
