Feature: Multi-Version Language Support
  As a developer
  I want to use different language versions
  So that I can match my project's requirements

  @version
  Scenario: Python 3.11 creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-python-311" using template "python" with version "3.11"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Python 3.11 should be installed in the container

  @version
  Scenario: Python 3.10 creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-python-310" using template "python" with version "3.10"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Python 3.10 should be installed in the container

  @version
  Scenario: Node.js 20 creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-nodejs-20" using template "nodejs" with version "20"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Node.js 20 should be installed in the container

  @version
  Scenario: Node.js 18 creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-nodejs-18" using template "nodejs" with version "18"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Node.js 18 should be installed in the container

  @version
  Scenario: Rust stable creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-rust-stable" using template "rust" with version "stable"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Rust should be installed in the container

  @version
  Scenario: Go 1.22 creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-go-122" using template "go" with version "1.22"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Go should be installed in the container

  @version
  Scenario: Go 1.21 creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-go-121" using template "go" with version "1.21"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Go should be installed in the container
