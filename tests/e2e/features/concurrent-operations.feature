Feature: Concurrent Project Creation
  As a developer
  I want to create multiple projects simultaneously
  So that I can quickly set up multiple development environments

  @concurrent
  Scenario: Multiple projects can be created in same workspace
    Given I am using the "shell-script" generator
    When I create projects named "test-concurrent-1", "test-concurrent-2", "test-concurrent-3" using template "python" simultaneously
    Then all projects should be created successfully
    And each project should have independent structure

  @concurrent
  Scenario: Different templates can be created simultaneously
    Given I am using the "shell-script" generator
    When I create "test-python" using template "python" and "test-nodejs" using template "nodejs" simultaneously
    Then both projects should be created successfully

  @concurrent
  Scenario: Same project name cannot be created twice
    Given I am using the "shell-script" generator
    And project "test-duplicate" exists
    When I attempt to create "test-duplicate" again
    Then creation should fail or handle duplicate appropriately
