table! {
    acronyms (id) {
        id -> Int4,
        key -> Varchar,
        regex -> Varchar,
        value -> Text,
    }
}

table! {
    occurances (thread_id, acronym_id, comment_id) {
        thread_id -> Varchar,
        comment_id -> Varchar,
        acronym_id -> Int4,
    }
}

joinable!(occurances -> acronyms (acronym_id));

allow_tables_to_appear_in_same_query!(
    acronyms,
    occurances,
);
