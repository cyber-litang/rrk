use super::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub class: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub class: &'a str,
}
