#!/usr/bin/env jq -f

# Downgrade OpenAPI version from 3.1.0 to 3.0.3 for progenitor compatibility.
# Progenitor does not support OpenAPI 3.1.0.
# The 3.1-specific features we use (nullable type arrays) are already handled
# by patch 15-fix-nullable-types.jq.

.openapi = "3.0.3"
