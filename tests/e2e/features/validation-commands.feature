# -*- coding: utf-8 -*-
Feature: Validation and Diff Commands
  Test project validation and template diff functionality

  @validation
  Scenario: Quick validation
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-quick" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --quick"
    Then validation should pass
    And output should show "validation passed" or "no errors"

  @validation
  Scenario: Full validation
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-full" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate"
    Then validation should pass
    And all validation checks should run

  @validation
  Scenario: Validation with JSON format
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-json" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --format json"
    Then output should be valid JSON

  @validation
  Scenario: Validation with SARIF format
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-sarif" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --format sarif"
    Then output should be valid JSON

  @validation
  Scenario: Validation with verbose output
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-verbose" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --verbose"
    Then validation should pass
    And output should contain detailed information

  @validation
  Scenario: Validation with warnings as errors
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-warnings" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --warnings-as-errors"
    Then validation should pass

  @diff
  Scenario: Show differences with template
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-basic" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde diff"
    Then differences should be shown
    And output should be formatted

  @diff
  Scenario: Diff with JSON format
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-json" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde diff --format json"
    Then output should be valid JSON

  @diff
  Scenario: Diff specific file
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-file" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde diff --file devcontainer.json"
    Then only devcontainer.json differences should be shown

  @diff
  Scenario: Diff with context lines
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-context" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    And I modify devcontainer.json
    When I run "isolde diff --context 5"
    Then differences with context should be shown

  @diff
  Scenario: Diff after modifications
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-modified" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    And I modify the project configuration
    When I run "isolde diff"
    Then configuration changes should be shown

  @validation
  Scenario: Validate specific path
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-path" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --path ."
    Then validation should pass

  @validation
  Scenario: Validate with custom config
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-custom" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    And I add custom validation rules
    When I run "isolde validate"
    Then custom validation should run

  @diff
  Scenario: Diff shows no changes when unmodified
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-no-changes" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde diff"
    Then output should indicate no changes or show minimal differences

  @validation @error
  Scenario: Validate detects syntax errors
    Given I have a project with syntax errors in devcontainer.json
    When I run "isolde validate"
    Then validation should fail with error containing "invalid" or "syntax" or "parse"

  @validation @error
  Scenario: Validate detects missing required files
    Given I have a project with missing required files
    When I run "isolde validate"
    Then validation should fail or show warnings

  @diff
  Scenario: Diff with multiple files
    Given I am using the "shell-script" generator
    When I create a project named "test-diff-multi" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    And I modify multiple configuration files
    When I run "isolde diff"
    Then all file differences should be shown

  @validation
  Scenario: Validate with quick mode skips build
    Given I am using the "shell-script" generator
    When I create a project named "test-validate-quick-skip" using template "python"
    Then the project should be created successfully
    And I run "isolde sync"
    When I run "isolde validate --quick"
    Then build test should be skipped
    And validation should complete quickly
