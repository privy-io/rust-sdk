#!/usr/bin/env jq -f

# Remove duplicate values from all enum lists in the schema
# Issue: Some enum definitions contain duplicate values (such as usdc) which cause code generation errors
# Solution: Recursively walk through the entire schema and deduplicate all enum arrays
#
# This is a general-purpose fix that ensures all enum lists throughout the OpenAPI spec
# contain only unique values. It uses jq's walk function to traverse the entire object
# tree and applies deduplication to any "enum" array it encounters.

walk(
  if type == "object" and has("enum") and (.enum | type == "array")
  then .enum |= unique
  else .
  end
)
