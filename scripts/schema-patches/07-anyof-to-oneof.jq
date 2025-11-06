#!/usr/bin/env jq -f

# Convert all anyOf to oneOf
# Issue: The spec uses anyOf where oneOf is more appropriate, being unintentionally
#        loose with type definitions
# Solution: Replace all instances of anyOf with oneOf
#
# anyOf allows objects to match one or more schemas (union), while oneOf requires
# exactly one schema to match (exclusive). Since the API expects exactly one variant,
# oneOf produces better generated code with proper type discrimination.
#
# This will be fixed upstream and removed.

walk(
  if type == "object" and has("anyOf")
  then . + {"oneOf": .anyOf} | del(.anyOf)
  else .
  end
)
