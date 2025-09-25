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
)
