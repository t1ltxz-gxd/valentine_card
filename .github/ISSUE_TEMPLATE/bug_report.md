---
name: "Bug Report"
description: "Report a bug to help us improve."
labels: [ "bug", "needs triage" ]
---
body:
- type: markdown
  attributes:
  value: |
  ## Bug Report

      Please fill out the following details to help us resolve the issue.

- type: input
  id: title
  attributes:
  label: "Title"
  description: "A brief summary of the bug."
  placeholder: "Enter the bug title here..."

- type: textarea
  id: description
  attributes:
  label: "Description"
  description: "A detailed description of the bug."
  placeholder: "Describe the bug in detail..."
  value: ""

- type: textarea
  id: steps
  attributes:
  label: "Steps to Reproduce"
  description: "The steps to reproduce the bug."
  placeholder: "1. Go to...\n2. Click on...\n3. See error..."
  value: ""

- type: textarea
  id: expected
  attributes:
  label: "Expected Behavior"
  description: "What you expected to happen."
  placeholder: "Describe the expected behavior..."
  value: ""

- type: textarea
  id: actual
  attributes:
  label: "Actual Behavior"
  description: "What actually happened."
  placeholder: "Describe what actually happened..."
  value: ""

- type: input
  id: environment
  attributes:
  label: "Environment"
  description: "The environment in which the bug occurred (e.g., OS, browser, version)."
  placeholder: "Enter the environment details here..."