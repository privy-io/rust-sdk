#!/usr/bin/env jq -f

# Fix OwnerInput schema by removing nullable case
# Issue: The raw spec includes a third anyOf case with "nullable: true" which causes
#        code generation issues with the type system. It is not clear what this is
#        supposed to mean, and the codegen just panics.
# Solution: Filter out the nullable case from the anyOf array
#
# This schema allows wallet/resource ownership to be specified in two ways:
#   1. By providing a P-256 public key (owner_id will be auto-generated)
#   2. By providing an existing user_id (must start with "did:privy:")
#
# The nullable case is removed to ensure proper type-safe handling in Rust.

.components.schemas.OwnerInput.anyOf |= map(select(.nullable != true))
