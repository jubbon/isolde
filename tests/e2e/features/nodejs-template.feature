Feature: Node.js Template
  As a developer
  I want to create a Node.js project with devcontainer
  So that I can start API development

  Scenario: Basic Node.js template creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-nodejs" using template "nodejs" with version "22"
    Then the project should be created successfully
    And the project should have dual git repositories
    And the devcontainer should build successfully
    And Node.js 22 should be installed in the container
    And npm should be available in the container
    And TypeScript should be installed

  Scenario: Node.js API preset includes TypeScript
    Given I am using the "shell-script" generator
    When I create a project named "test-api" using template "nodejs" with version "22" and preset "node-api"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And TypeScript should be installed
    And Vitest should be available
