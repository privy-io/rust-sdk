#!/usr/bin/env jq -f

# Fix Coinbase asset enums where lowercase "eth" appears alongside uppercase "ETH".
# These collide when converted to Rust enum variants (both become "Eth").
# Solution: Remove the lowercase "eth" variant, keeping only "ETH".

if .components.schemas.CoinbaseEthereumAsset.enum then
  .components.schemas.CoinbaseEthereumAsset.enum |= map(select(. != "eth"))
else
  .
end
|
if .components.schemas.CoinbaseOnRampInitInput.oneOf[0].properties.assets.items.enum then
  .components.schemas.CoinbaseOnRampInitInput.oneOf[0].properties.assets.items.enum |= map(select(. != "eth"))
else
  .
end
