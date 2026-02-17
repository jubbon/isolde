Feature: Configuration Options
  As a developer
  I want to configure proxy and Claude provider settings
  So that I can customize the devcontainer for my environment

  @config
  Scenario: Proxy settings should be applied
    Given I am using the "shell-script" generator
    When I create a project named "test-proxy" using template "python" with HTTP proxy "http://proxy.example.com:8080"
    Then the project should be created successfully
    And proxy configuration should exist in devcontainer

  @config
  Scenario: Claude provider option should be applied
    Given I am using the "shell-script" generator
    When I create a project named "test-provider" using template "python" with Claude provider "anthropic"
    Then the project should be created successfully
    And Claude provider should be configured

  @config
  Scenario: Custom Claude version should be applied
    Given I am using the "shell-script" generator
    When I create a project named "test-claude-version" using template "python" with Claude version "1.2.3"
    Then the project should be created successfully
    And Claude version should be configured

  @config
  Scenario: Both proxy and provider options should work together
    Given I am using the "shell-script" generator
    When I create a project named "test-combo" using template "python" with Claude provider "anthropic" and HTTP proxy "http://proxy.example.com:8080"
    Then the project should be created successfully
    And both configurations should be applied
