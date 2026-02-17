Feature: Edge Cases and Negative Tests
  As a developer
  I want the system to handle edge cases gracefully
  So that I get clear error messages and predictable behavior

  @edge-case @negative
  Scenario: Invalid template name should fail gracefully
    Given I am using the "shell-script" generator
    When I attempt to create a project named "test-invalid" using template "nonexistent"
    Then project creation should fail
    And error message should mention invalid template

  @edge-case @negative
  Scenario: Invalid preset should fail gracefully
    Given I am using the "shell-script" generator
    When I attempt to create a project named "test-invalid-preset" using template "python" with preset "nonexistent-preset"
    Then project creation should fail
    And error message should mention invalid preset

  @edge-case @negative
  Scenario: Invalid language version should fail gracefully
    Given I am using the "shell-script" generator
    When I attempt to create a project named "test-invalid-version" using template "python" with version "99.99"
    Then project creation should fail
    And error message should mention invalid version

  @edge-case @negative
  Scenario: Project name with spaces should be rejected
    Given I am using the "shell-script" generator
    When I attempt to create a project named "test project with spaces" using template "python"
    Then project creation should fail
    And error message should mention "Project name can only contain"

  @edge-case
  Scenario: Project name with dashes should work
    Given I am using the "shell-script" generator
    When I create a project named "test-project-with-dashes" using template "python"
    Then the project should be created successfully
    And the devcontainer should build successfully

  @edge-case
  Scenario: Project name with underscores should work
    Given I am using the "shell-script" generator
    When I create a project named "test_project_with_underscores" using template "python"
    Then the project should be created successfully
    And the devcontainer should build successfully

  @edge-case
  Scenario: Project name with numbers should work
    Given I am using the "shell-script" generator
    When I create a project named "test123project456" using template "python"
    Then the project should be created successfully
    And the devcontainer should build successfully

  @edge-case
  Scenario: Very long project name should be handled
    Given I am using the "shell-script" generator
    When I create a project named "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" using template "python"
    Then the project should be created successfully

  @edge-case @negative
  Scenario: Empty project name should fail
    Given I am using the "shell-script" generator
    When I attempt to create a project named "" using template "python"
    Then project creation should fail

  @edge-case @negative
  Scenario: Project name with unicode characters should be rejected
    Given I am using the "shell-script" generator
    When I attempt to create a project named "test-проект" using template "python"
    Then project creation should fail
    And error message should mention "Project name can only contain"

  @edge-case
  Scenario: Creating project in existing directory should fail or overwrite
    Given I am using the "shell-script" generator
    And a project named "test-existing" already exists
    When I attempt to create a project named "test-existing" using template "python"
    Then project creation should handle existing directory appropriately

  @edge-case
  Scenario: Single character project name should work
    Given I am using the "shell-script" generator
    When I create a project named "a" using template "python"
    Then the project should be created successfully
