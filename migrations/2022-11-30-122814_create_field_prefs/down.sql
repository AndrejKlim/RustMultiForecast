-- This file should undo anything in `up.sql`
drop function jsonb_merge_recurse;
drop function jsonb_merge_recurse_all;
drop table user_field_preferences;
drop table forecast_field_user_preferences;
