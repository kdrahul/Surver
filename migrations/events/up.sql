-- Your SQL goes here
create table event(
    id serial not null primary key,
    name varchar(30) not null,
    description text,
    organizers varchar(20),
    starts_at timestamp,
    max_participants smallint,
    fee integer
);
