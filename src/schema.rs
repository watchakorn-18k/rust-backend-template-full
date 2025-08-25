// @generated automatically by Diesel CLI.

diesel::table! {
    post (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    post,
    users,
);
