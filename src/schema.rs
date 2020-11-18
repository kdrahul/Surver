table! {
    ersusers (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        branch -> Varchar,
        user_group -> Varchar,
        joined_on -> Timestamp,
    }
}

table! {
    event (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        organizers -> Nullable<Varchar>,
        starts_at -> Nullable<Timestamp>,
        max_participants -> Nullable<Int2>,
        fee -> Nullable<Int4>,
    }
}

table! {
    question (id) {
        id -> Int4,
        question_title -> Nullable<Text>,
        option1 -> Nullable<Varchar>,
        option2 -> Nullable<Varchar>,
        option3 -> Nullable<Varchar>,
        option4 -> Nullable<Varchar>,
        event_id -> Int4,
    }
}

joinable!(question -> event (event_id));

allow_tables_to_appear_in_same_query!(
    ersusers,
    event,
    question,
);
