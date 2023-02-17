-- This file should undo anything in `up.sql`
drop table user_field_preferences;
drop table forecast_field_user_preferences;
alter table users drop column last_command;