#!/usr/bin/env jq -f

# Remove deprecated objects from the schema
# Issue: Deprecated objects clutter the generated code and may cause confusion
# Solution: Walk through the entire schema and replace any object marked as deprecated with null
#
# This keeps the generated SDK clean by excluding deprecated endpoints and schemas
# that should no longer be used. The nulls will be cleaned up by jq's normal processing.

walk(
  if type == "object" and .deprecated
  then null
  else .
  end
)
