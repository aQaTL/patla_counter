-- Your SQL goes here

create table passwords(
    id serial primary key,
    password_hash char(32) not null
);