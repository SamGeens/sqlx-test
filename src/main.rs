use std::error::Error;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use entities::prelude::*;
mod entities;

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // This line loads the environment variables from the ".env" file.
  dotenv().ok();
  
  let pool = PgPoolOptions::new()
  .max_connections(5)
  .connect(&std::env::var("DATABASE_URL").expect("missing environment variable DATABASE_URL")).await?;

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
  .fetch_all(&pool)
  .await?;

  // This one works
  // let persons = sqlx::query_as!(Person,
  //   r#"
  //   SELECT
  //     P.id, P.name, P.age, P.isActive,
  //     ARRAY_AGG((L.id, L.name)) AS "language: Vec<Language>"
  //   FROM person P
  //     JOIN language L on P.id = L.PERSON_id
  //   group by P.id, P.name, P.age, P.isActive;
  //   "#
  // )
  // .fetch_all(&pool)
  // .await?;

  println!("{:?}", persons);

  Ok(())
}
