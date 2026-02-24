Feature: Python Template
  As a developer
  I want to create a Python project with devcontainer
  So that I can start coding immediately

  Scenario: Basic Python template creates valid project
    Given I am using the "shell-script" generator
    When I create a project named "test-python" using template "python" with version "3.12"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Python 3.12 should be installed in the container
    And uv should be available in the container
    And pytest should be available in the container

  Scenario: Python ML preset includes Jupyter
    Given I am using the "shell-script" generator
    When I create a project named "test-ml" using template "python" with version "3.12" and preset "python-ml"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And Jupyter should be installed in the container
    And numpy should be importable
    And pandas should be importable

  Scenario: Python web preset includes web tools
    Given I am using the "shell-script" generator
    When I create a project named "test-web" using template "python" with version "3.12" and preset "python-web"
    Then the project should be created successfully
    And the devcontainer should build successfully
    And pytest should be available in the container
