# SQLx test

This repo is a test repo for a problem I stumbled upon while using rust [sqlx](https://github.com/launchbadge/sqlx).

## Usage

Create a postgres database and run the commands from `database.sql` to create the schema and test data. Remove ´.example´ from ´.env.example´ and adjust the database url to your needs.

## The error

**When trying to query all persons with their coresponding languages as such:**
```rust
  let persons = sqlx::query_as::<_, Person>(
    r#"
    SELECT
      P.id, P.name, P.age, P.isActive,
      ARRAY_AGG((L.id, L.name)) AS "language"
    FROM person P
      JOIN language L on P.id = L.PERSON_id
    group by P.id, P.name, P.age, P.isActive;
    "#
  )
  .fetch_one(&pool)
  .await?;

  println!("{:?}", persons);
```

**I get the following error:**
```rust
Error: ColumnDecode { index: "\"language\"", source: "mismatched types; Rust type `alloc::vec::Vec<sqlx_test::entities::language::Language>` (as SQL type `_language`) is not compatible with SQL type `RECORD[]`" }
error: process didn't exit successfully: `target\debug\sqlx-test.exe` (exit code: 1)
```

I had to the following piece of code because otherwise I get a PgHasArrayType error:
```rust
impl sqlx::postgres::PgHasArrayType for Language {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
      // Array type name is the name of the element type prefixed with `_`.
      sqlx::postgres::PgTypeInfo::with_name("_language")
    }
}
```

## Difference

However when I use the following query no error happens and the `PgHasArrayType` isn't even needed.
```rust
let persons = sqlx::query_as!(Person,
  r#"
  SELECT
    P.id, P.name, P.age, P.isActive,
    ARRAY_AGG((L.id, L.name)) AS "language: Vec<Language>"
  FROM person P
    JOIN language L on P.id = L.PERSON_id
  group by P.id, P.name, P.age, P.isActive;
  "#
)
.fetch_all(&pool)
.await?;

println!("{:?}", persons);
```

Why does this implementation of the query works while I can't get the previous working?
