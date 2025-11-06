#!/usr/bin/env jq -f

# Remove additionalProperties: false from all schemas
# Issue: The spec uses additionalProperties: false inappropriately in many places,
#        which causes overly strict validation and code generation issues
# Solution: Remove all instances of additionalProperties: false
#
# Note: This is a workaround for loose schema definitions. The proper fix would be
# to correctly define which schemas should and shouldn't allow additional properties,
# but for now we remove all restrictions to allow more flexible client code.
#
# This will be fixed upstream and removed.

walk(
  if type == "object" and .additionalProperties == false
  then del(.additionalProperties)
  else .
  end
)
