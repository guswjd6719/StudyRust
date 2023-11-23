// main.rs
mod entities;

use futures::executor::block_on;
use sea_orm::*;

use entities::{prelude::*, *};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "mysql://user:password@localhost:3306";
const DB_NAME: &str = "bakery";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
   db.execute(Statement::from_string(
                 db.get_database_backend(),
                  format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
              ))
              .await?;
   
              let url = format!("{}/{}", DATABASE_URL, DB_NAME);
              Database::connect(&url).await?
          }
                 DbBackend::Postgres => {
             db.execute(Statement::from_string(
                  db.get_database_backend(),
                  format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
              ))
              .await?;
              db.execute(Statement::from_string(
                  db.get_database_backend(),
                  format!("CREATE DATABASE \"{}\";", DB_NAME),
              ))
              .await?;
   
              let url = format!("{}/{}", DATABASE_URL, DB_NAME);
              Database::connect(&url).await?
          }
          DbBackend::Sqlite => db,
      };

      // Finding by id is built-in
      let mut happy_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
      //assert_eq!(chef.unwrap().name, "John");
      dbg!(happy_bakery);

    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}