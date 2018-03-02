use schema::occurances;

#[derive(Queryable, Identifiable)]
#[primary_key(thread_id, acronym_id, comment_id)]
pub struct Occurance {
    pub thread_id: String,
    pub comment_id: String,
    pub acronym_id: i32,
}

use schema::acronyms;

#[derive(Queryable, Identifiable, Debug)]
pub struct Acronym {
    pub id: i32,
    pub key: String,
    pub regex: String,
    pub value: String,
}
