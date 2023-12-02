use ::migration::{Migrator, MigratorTrait};
use dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection as DbConn, DbErr};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), String> {
    let db = establish_connection().await.unwrap();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        panic!("No args provided")
    }
    if args[1] == "up" {
        println!("run up migrations ...");
        Migrator::up(&db, None).await.unwrap();
        println!("Migrated.");
    }
    if args[1] == "down" {
        println!("run down migrations ...");
        Migrator::reset(&db).await.unwrap();
        println!("Migrated.");
    }
    if args[1] == "fresh" {
        println!("run fresh migrations ...");
        Migrator::fresh(&db).await.unwrap();
        println!("Migrated.");
    }
    Ok(())
}

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv::from_filename(".env").ok();
    // get database url from env
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // init connection options
    let mut options = ConnectOptions::new(db_url);
    options
        .max_connections(3)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5));
    // get db connection
    let db_res = Database::connect(options).await;
    // check for errors
    match db_res {
        Ok(db_conn) => {
            println!("Database connection established.");
            Ok(db_conn)
        }
        Err(db_err) => Err(db_err),
    }
}
