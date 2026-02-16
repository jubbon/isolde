Feature: VS Code Compatibility
  As a developer
  I want the devcontainer to work in VS Code

  Scenario: devcontainer.json is valid for Python template
    Given I am using the "shell-script" generator
    When I create a project named "test-vscode-python" using template "python" with version "3.12"
    Then the project should be created successfully
    And devcontainer.json should be valid JSON
    And devcontainer.json should contain field "image"
    And devcontainer.json should specify VS Code extensions
    And the devcontainer should build successfully
    And claude command should exist in the container
    And claude --version command should work

  Scenario: devcontainer.json is valid for Node.js template
    Given I am using the "shell-script" generator
    When I create a project named "test-vscode-nodejs" using template "nodejs" with version "22"
    Then the project should be created successfully
    And devcontainer.json should be valid JSON
    And devcontainer.json should contain field "image"
    And devcontainer.json should specify VS Code extensions
