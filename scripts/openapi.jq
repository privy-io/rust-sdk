.[1] * .[0] | walk(
  if type == "array"
  then map(select(type != "object" or .name != "privy-app-id"))
  else .
  end
) | walk(
  if type == "object" and .deprecated
  then null
  else .
  end
) | walk(
  # we use additionalProperties inappropriate almost everywhere so just remove it
  if type == "object" and .additionalProperties == false
  then del(.additionalProperties)
  else .
  end
)
