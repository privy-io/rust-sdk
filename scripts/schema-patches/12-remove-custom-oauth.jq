#!/usr/bin/env jq -f

# Remove LinkedAccountCustomOauth from LinkedAccount oneOf
# Issue: The allOf structure for LinkedAccountCustomOauth makes it impossible
#        to infer a variant name for code generation
# Solution: Remove this variant from LinkedAccount.oneOf entirely

.components.schemas.LinkedAccount.oneOf |= map(
  select(
    # Keep everything except the allOf that references LinkedAccountCustomOauth
    if type == "object" and has("allOf") and
       (.allOf[0] | has("$ref") and (.["$ref"] == "#/components/schemas/LinkedAccountCustomOauth"))
    then false
    else true
    end
  )
)
