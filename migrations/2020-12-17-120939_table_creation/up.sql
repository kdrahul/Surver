-- Your SQL goes here

create table Users (
    
    id serial primary key not null,
    username varchar(20), 
    password Text,
    role varchar(10),

    first_name varchar(20),
    last_name varchar(20),
    email varchar(30),
    phone varchar(13),
    branch varchar(4)
);

create table Events (

    id serial primary key not null,
    name varchar(50),

    description Text,
    venue varchar(100),
    starts_at timestamp,
    max_limit integer,
    fee integer,
    prize_money integer
);

create table Questions (
    id serial not null primary key,
    question_description varchar(200),
    event_id serial,
    foreign key(event_id) references Events(id)
);

create table Response (
    id serial primary key,
    user_id serial,
    event_id serial,
    question_id serial,
    response_date timestamp,
    user_response varchar(50),

    foreign key(user_id) references users(id),
    foreign key(event_id) references Events(id),
    foreign key(question_id) references questions(id)

);

