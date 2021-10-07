// Generated with `diesel print-schema` .env necessary

table! {
    names (name_id) {
        name_id -> Int8,
        name -> Varchar,
        date -> Date,
        time -> Time,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    names,
    users,
);