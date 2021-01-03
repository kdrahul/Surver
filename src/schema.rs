table! {
    events (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        venue -> Nullable<Varchar>,
        starts_at -> Nullable<Timestamp>,
        max_limit -> Nullable<Int4>,
        fee -> Nullable<Int4>,
        prize_money -> Nullable<Int4>,
    }
}

table! {
    questions (id) {
        id -> Int4,
        question -> Nullable<Varchar>,
        option1 -> Nullable<Varchar>,
        option2 -> Nullable<Varchar>,
        option3 -> Nullable<Varchar>,
        option4 -> Nullable<Varchar>,
        answer -> Nullable<Varchar>,
        event_id -> Int4,
    }
}

table! {
    response (id) {
        id -> Int4,
        user_id -> Int4,
        event_id -> Int4,
        question_id -> Int4,
        user_response -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        password -> Nullable<Text>,
        role -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        branch -> Nullable<Varchar>,
    }
}

joinable!(questions -> events (event_id));
joinable!(response -> events (event_id));
joinable!(response -> questions (question_id));
joinable!(response -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    questions,
    response,
    users,
);
