// @generated automatically by Diesel CLI.

diesel::table! {
    script_tag_assoc (id) {
        id -> Int4,
        tag_id -> Int4,
        is_output -> Bool,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(script_tag_assoc -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    script_tag_assoc,
    tags,
);
