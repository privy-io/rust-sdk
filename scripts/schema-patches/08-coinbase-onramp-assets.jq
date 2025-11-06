#!/usr/bin/env jq -f

# Fix CoinbaseOnRampInitInput schema
# Issue: lowercase "eth" appears in the spec alongside "ETH"
# Solution: Filter out the lowercase "eth" variant
#
# This patch removes the lowercase "eth" from the assets enum in the
# CoinbaseOnRampInitInput schema, keeping only the uppercase "ETH" variant.

.components.schemas.CoinbaseOnRampInitInput.oneOf[0].properties.assets.items.enum |= map(select(. != "eth"))
