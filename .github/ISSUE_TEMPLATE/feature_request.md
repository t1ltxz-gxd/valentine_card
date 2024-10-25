---
name: "Feature Request"
description: "Suggest a new feature or enhancement."
labels: [ "enhancement", "feature request" ]
---
body:
- type: markdown
  attributes:
  value: |
  ## Feature Request

      Please provide the following details to help us understand your request.

- type: input
  id: title
  attributes:
  label: "Title"
  description: "A brief summary of the feature request."
  placeholder: "Enter the feature title here..."

- type: textarea
  id: description
  attributes:
  label: "Description"
  description: "A detailed description of the feature request."
  placeholder: "Describe the feature in detail..."
  value: ""

- type: textarea
  id: motivation
  attributes:
  label: "Motivation"
  description: "Explain why this feature is needed."
  placeholder: "Describe the motivation behind this feature..."
  value: ""

- type: textarea
  id: alternatives
  attributes:
  label: "Alternatives"
  description: "Describe any alternative solutions or features you've considered."
  placeholder: "Describe any alternatives you've considered..."
  value: ""

- type: input
  id: additional_context
  attributes:
  label: "Additional Context"
  description: "Any other context or information."
  placeholder: "Enter any additional context here..."