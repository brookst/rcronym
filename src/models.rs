#[derive(Queryable)]
pub struct Acronym {
    pub id: i32,
    pub key: String,
    pub regex: String,
    pub value: String,
}
