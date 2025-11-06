#!/usr/bin/env jq -f

# Remove endpoints that use multipart/form-data
# Issue: Progenitor doesn't support multipart/form-data request bodies
# Solution: Remove these endpoints from the spec to avoid code generation errors
#
# Currently affected endpoints:
# - POST /v1/kraken_embed/users/{user_id}/verifications
#
# This walks through all paths and filters out any operations (GET, POST, etc.)
# that have multipart/form-data in their requestBody content types.

walk(
  if type == "object" and has("requestBody") and .requestBody.content["multipart/form-data"]
  then del(.requestBody.content["multipart/form-data"])
  else .
  end
)
