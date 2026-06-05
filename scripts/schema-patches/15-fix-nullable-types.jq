#!/usr/bin/env jq -f

# Fix OpenAPI 3.1 nullable type arrays for progenitor compatibility.
# OpenAPI 3.1 uses "type": ["string", "null"] to indicate nullable types.
# Progenitor expects OpenAPI 3.0 style "type": "string" (it infers nullability
# from whether the field is in the required array).
#
# This patch:
# 1. For objects with properties: removes nullable fields from `required`
#    so progenitor generates Option<T>
# 2. Converts "type": ["X", "null"] to "type": "X"
# 3. Removes {"type": "null"} entries from oneOf/anyOf arrays

# Helper: check if a property schema is nullable.
# A property is nullable if:
# - Its type is an array containing "null" (e.g. ["string", "null"])
# - Any entry in its allOf/anyOf/oneOf has type containing "null"
def is_nullable:
  if (.type // null | type) == "array" and (.type | contains(["null"])) then true
  elif (.allOf // [] | any(.type // null | type == "array" and contains(["null"]))) then true
  elif (.anyOf // [] | any(.type // null | type == "array" and contains(["null"]))) then true
  elif (.oneOf // [] | any(.type // null | type == "array" and contains(["null"]))) then true
  else false end;

# Pass 1: Remove nullable fields from required arrays.
# Must run before type flattening so we can detect ["string", "null"].
walk(
  if type == "object" and .properties and .required then
    . as $obj |
    .required |= map(
      select(
        . as $field |
        ($obj.properties[$field] | is_nullable) | not
      )
    )
  else . end
)
# Pass 2: Flatten nullable type arrays and clean up oneOf/anyOf.
| walk(
  if type == "object" then
    (if (.type | type) == "array" then
      .type = (.type | map(select(. != "null")) | if length > 0 then first else "object" end)
    elif .type == "null" then
      .type = "string"
    else . end)
    | (if .oneOf then .oneOf |= map(select(.type != "null")) else . end)
    | (if .anyOf then .anyOf |= map(select(.type != "null")) else . end)
  else
    .
  end
)
