@isolation
Feature: Isolation levels
  Isolation levels control how much host Claude Code state is shared with devcontainers.

  Scenario: None isolation mounts entire host ~/.claude
    Given I am using the "shell-script" generator
    When I create a project with isolation level "none"
    Then the project should be created successfully
    And devcontainer.json should have 5 mounts
    And no mount should reference ".isolde/volumes"

  Scenario: Session isolation overlays sessions and statsig
    Given I am using the "shell-script" generator
    When I create a project with isolation level "session"
    Then the project should be created successfully
    And devcontainer.json should have 8 mounts
    And a mount should reference "claude-sessions"
    And a mount should reference "claude-statsig"
    And a mount should reference "omc-config.json"

  Scenario: Workspace isolation adds plugins overlay
    Given I am using the "shell-script" generator
    When I create a project with isolation level "workspace"
    Then the project should be created successfully
    And devcontainer.json should have 9 mounts
    And a mount should reference "claude-plugins"
    And a mount should reference "claude-sessions"

  Scenario: Full isolation uses local claude-home
    Given I am using the "shell-script" generator
    When I create a project with isolation level "full"
    Then the project should be created successfully
    And a mount should reference "claude-home"
    And no mount should reference "source=${localEnv:HOME}/.claude,"
