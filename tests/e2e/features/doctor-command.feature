# -*- coding: utf-8 -*-
Feature: Doctor Command
  Test system diagnostics and health checks for Isolde

  @doctor
  Scenario: Run full diagnostics
    When I run "isolde doctor"
    Then all components should be checked
    And report should be generated
    And exit code should indicate success or issues

  @doctor
  Scenario: Check specific component - Docker
    When I run "isolde doctor --component docker"
    Then only Docker should be checked
    And Docker status should be reported

  @doctor
  Scenario: Check specific component - devcontainers
    When I run "isolde doctor --component devcontainers"
    Then only devcontainers CLI should be checked
    And devcontainers status should be reported

  @doctor
  Scenario: Check specific component - Claude
    When I run "isolde doctor --component claude"
    Then only Claude Code CLI should be checked
    And Claude status should be reported

  @doctor
  Scenario: Generate diagnostic report
    When I run "isolde doctor --report /tmp/doctor-report.json"
    Then report file should be created
    And report should be valid JSON

  @doctor
  Scenario: Verbose diagnostic output
    When I run "isolde doctor --verbose"
    Then detailed diagnostic information should be shown
    And all component versions should be displayed

  @doctor
  Scenario: Doctor with fix option
    When I run "isolde doctor --fix"
    Then automatic fixes should be attempted
    And fix results should be reported

  @doctor
  Scenario: Check all components individually
    When I run "isolde doctor --component docker"
    Then Docker component check should complete
    When I run "isolde doctor --component devcontainers"
    Then devcontainers component check should complete
    When I run "isolde doctor --component claude"
    Then Claude component check should complete

  @doctor
  Scenario: Doctor with missing component
    Given a component is not installed
    When I run "isolde doctor"
    Then missing component should be reported
    And installation instructions should be provided

  @doctor
  Scenario: Doctor report format
    When I run "isolde doctor --report /tmp/doctor-report.txt"
    Then report should be human-readable
    When I run "isolde doctor --report /tmp/doctor-report.json"
    Then report should be machine-readable

  @doctor
  Scenario: Doctor checks template system
    When I run "isolde doctor --component templates"
    Then template availability should be checked
    And template status should be reported

  @doctor
  Scenario: Doctor checks feature system
    When I run "isolde doctor --component features"
    Then feature availability should be checked
    And core features should be listed

  @doctor
  Scenario: Doctor with all components
    When I run "isolde doctor --component all"
    Then all components should be checked
    And comprehensive report should be generated

  @doctor
  Scenario: Doctor exit codes
    When I run "isolde doctor"
    Then exit code 0 should indicate all is well
    When a component has issues
    Then exit code should indicate problems

  @doctor
  Scenario: Doctor quick check
    When I run "isolde doctor --quick"
    Then essential components should be checked
    And optional components should be skipped

  @doctor
  Scenario: Doctor with JSON output
    When I run "isolde doctor --format json"
    Then output should be valid JSON
    And all component statuses should be included

  @doctor
  Scenario: Doctor checks configuration
    When I run "isolde doctor --component config"
    Then configuration files should be checked
    And configuration issues should be reported

  @doctor
  Scenario: Doctor checks network connectivity
    When I run "isolde doctor --component network"
    Then network connectivity should be checked
    And proxy settings should be verified

  @doctor
  Scenario: Doctor with fix Dry Run
    When I run "isolde doctor --fix --dry-run"
    Then potential fixes should be listed
    But no changes should be made
