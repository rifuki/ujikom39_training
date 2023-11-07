use crate::schema;
use diesel::{Queryable, Selectable, prelude::Insertable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = schema::pembeli)]
pub struct Pembeli {
    pub id_pembeli: i32,
    pub nama_pembeli: String,
    pub jk: String,
    pub no_tlp: String,
    pub alamat: String
}