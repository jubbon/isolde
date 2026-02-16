Feature: VS Code Dev Containers CLI
  As a developer
  I want the devcontainer to work with VS Code Dev Containers CLI
  So that it works identically to VS Code

  @cli
  Scenario: Dev container starts successfully for Python
    Given I am using the "shell-script" generator
    When I create a project named "test-cli-python" using template "python" with version "3.12"
    Then the project should be created successfully
    When I start the devcontainer for the project
    Then postCreateCommand should have executed successfully
    And devcontainer features should be installed
    And I can execute commands in the devcontainer

  @cli
  Scenario: Dev container starts successfully for Node.js
    Given I am using the "shell-script" generator
    When I create a project named "test-cli-nodejs" using template "nodejs" with version "22"
    Then the project should be created successfully
    When I start the devcontainer for the project
    Then Node.js should be available in the devcontainer
    And I can execute commands in the devcontainer
    When I stop the devcontainer
    Then the container should be removed
