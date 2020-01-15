-- This file should undo anything in `up.sql`

alter table entries drop column counter_id;

drop table counters;
