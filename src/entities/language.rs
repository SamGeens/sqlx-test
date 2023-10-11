use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, sqlx::Type)]
pub struct Language {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "name")]
    pub name: String,
}

// impl sqlx::postgres::PgHasArrayType for Language {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//       // Array type name is the name of the element type prefixed with `_`.
//       sqlx::postgres::PgTypeInfo::with_name("_language")
//     }
// }

impl sqlx::postgres::PgHasArrayType for Language {
  fn array_type_info() -> sqlx::postgres::PgTypeInfo {
    sqlx::postgres::PgTypeInfo::with_name("_language")
  }

  fn array_compatible(_ty: &sqlx::postgres::PgTypeInfo) -> bool {
    true
  }
}
