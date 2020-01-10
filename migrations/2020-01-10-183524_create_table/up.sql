-- Your SQL goes here

create table entries(
    id serial primary key,
    reason text,
    created timestamp without time zone not null
);