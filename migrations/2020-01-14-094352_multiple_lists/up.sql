-- Your SQL goes here

create table counters (
    id serial primary key,
    name text
);

insert into counters values(1, 'Default');

alter table entries disable trigger user;

alter table entries
    add column counter_id serial not null;

update entries set counter_id=1;

alter table entries add constraint fk_counter_id foreign key (counter_id) references counters(id)
    on delete cascade;

alter table entries enable trigger user;
