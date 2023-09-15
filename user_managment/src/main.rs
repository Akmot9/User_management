use std::env;

use rocket::futures::future::Lazy;
use rocket::local;
use serde::Serialize;
use surrealdb::engine::local::RocksDb;
use surrealdb::engine::any::Any;
use surrealdb::opt::Config;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::{Ws,Wss};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Id;
use surrealdb::sql::Array;
use surrealdb::sql::Thing;
use surrealdb::sql::Value;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

static LOCAL: Lazy<Surreal<surrealdb::engine::any::Any>> = Lazy::new(Surreal::init);

fn get_path(path: &str) -> std::io::Result<String> {
  let mut dir = env::current_exe()?;
  dir.pop();
  dir.push(path);
  Ok(dir.display().to_string())
}


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let root = Root {
      username:"root",
      password:"root"
    };
    let db_path = get_path("mydatabase.db").expect("Couldn't find path");
    let path = format!("file://{}",db_path);
    LOCAL.connect(path.as_str()).await?;
    LOCAL.use_ns("root").use_db("root").await?;
    println!("fuck yes local passed");

    Ok(())
}