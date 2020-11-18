-- Your SQL goes here
create table ERSusers (     
    id serial not null primary key,
    name varchar(30) not null,
    email varchar(30) not null,
    phone varchar(13) not null,
    branch varchar(5) not null,
    user_group varchar(5) not null,
    joined_on timestamp not null
);
