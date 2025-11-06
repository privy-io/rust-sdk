#!/usr/bin/env jq -f

# Remove duplicate properties from schemas
# Issue: Some schemas have duplicate properties with different casing (e.g., first_name and firstName)
# Solution: Keep snake_case versions, remove camelCase versions
#
# Currently affected schemas:
# - LinkedAccountTelegram: has both first_name/firstName and telegram_user_id/telegramUserId
#
# This removes camelCase properties and their entries from the required array.

walk(
  if type == "object" and .title == "Telegram"
  then
    .properties |= del(.firstName, .telegramUserId) |
    if .required then
      .required |= map(select(. != "firstName" and . != "telegramUserId"))
    else .
    end
  else .
  end
)
