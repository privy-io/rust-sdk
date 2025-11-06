#!/usr/bin/env jq -f

# Fix LinkedAccountSolanaEmbeddedWallet required fields
# Issue: public_key is not being returned in some API responses
# Solution: Remove public_key from required fields list
#
# The API spec originally included public_key as a required field, but in practice
# some API responses do not include this field. This patch updates the schema to match
# the actual API behavior by filtering out public_key from the required array.

.components.schemas.LinkedAccountSolanaEmbeddedWallet.required |= map(select(. != "public_key"))
