//! ORM schema generated with `diesel print-schema` .env necessary
//! ```txt
//! DATABASE_URL=postgres://alejandro:252525@localhost/actix_test
//! ```
//!
//! Necessary to have ORM. Structs can be serialized and deserialized from them

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

allow_tables_to_appear_in_same_query!(
    names,
    users,
);
