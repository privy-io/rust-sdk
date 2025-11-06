#!/usr/bin/env jq -f

# Remove privy-app-id from operation parameters
# Issue: privy-app-id appears as a parameter in operation definitions, but it should be
#        handled as a security scheme instead (via HTTP headers)
# Solution: Walk through all arrays and filter out objects with name "privy-app-id"
#
# This will be fixed upstream by moving privy-app-id into the OpenAPI security scheme,
# but for now we need to strip it from the parameter lists to avoid duplicate definitions
# and ensure the generated client code uses the header-based authentication correctly.

walk(
  if type == "array"
  then map(select(type != "object" or .name != "privy-app-id"))
  else .
  end
)
