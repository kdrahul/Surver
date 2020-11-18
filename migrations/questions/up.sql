create table question(
    id serial not null primary key,
    question_title text,
    option1 varchar(30),
    option2 varchar(30),
    option3 varchar(30),
    option4 varchar(30),
    event_id serial references event(id)
);

