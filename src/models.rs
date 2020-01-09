use super::schema::rooms;

#[derive(Queryable)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "rooms"]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
