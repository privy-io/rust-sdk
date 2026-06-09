#!/usr/bin/env jq -f

# Clean up operations for progenitor compatibility:
# 1. Remove the OAuth token endpoint (internal auth flow not exposed by the SDK)
# 2. Remove null operations and operations missing the `responses` field
# 3. Remove non-2xx responses (progenitor asserts at most one response type per operation)
# 4. Remove paths that become empty after filtering

del(.paths["/api/oauth/v2/token"])
| .paths |= with_entries(
    .value |= with_entries(
      select(.value | type == "object" and has("responses") and .responses != null and .responses != {})
    )
    | .value |= with_entries(
      .value.responses |= with_entries(select(.key | test("^2")))
    )
  )
| .paths |= with_entries(select(.value != {}))
