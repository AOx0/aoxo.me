//! ORM schema generated with `diesel print-schema` .env necessary
//! ```txt
//! DATABASE_URL=postgres://alejandro:252525@localhost/actix_test
//! ```
//!
//! Necessary to have ORM. Structs can be serialized and deserialized from them
//!
//!

table! {
    missions (user_id) {
        mission1 -> Bool,
        mission2 -> Bool,
        mission3 -> Bool,
        mission4 -> Bool,
        mission5 -> Bool,
        user_id -> Int8,
    }
}

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
        name -> Varchar,
    }
}

joinable!(missions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    missions,
    names,
    users,
);
