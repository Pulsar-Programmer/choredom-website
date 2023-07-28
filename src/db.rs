use serde::{Serialize, Deserialize};
use serde_json::json;
use std::borrow::Cow;
use surrealdb::Surreal;
use surrealdb::sql;
use surrealdb::opt::auth::{Root, Scope};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb as s;

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}


async fn setup_users(db: &mut Db) -> s::Result<()> {

    // let credentials = Scope {
    //     namespace: "choredom",
    //     database: "main",
    //     scope: "user",
    //     params: Credentials {
    //         email: "info@surrealdb.com",
    //         pass: "123456",
    //     },
    // };


    //this function will be called from the setup_db function
    todo!()
    // Ok(db)
}

pub async fn setup_db() -> s::Result<Db>{
    //Create the db connection
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    // FOR NOW SIGNING IN AS ROOT USER IS OK, 
    //but later make sure you make different accounts 
    //each for different operations that make secure the db

    //config namespace and database
    db.use_ns("choredom").use_db("main").await?;

    // db.authenticate(jwt).await?;

    Ok(db)
}

pub type Db = Surreal<Client>;

//Create
pub async fn register<V: serde::Serialize>(db: &mut Db, table: &str, id: &str, value: V) -> s::Result<()>{
    db.create((table, id)).content(value).await?;
    Ok(())
}

//Read
pub async fn retrieve<V: serde::de::DeserializeOwned>(db: &mut Db, table: &str) -> s::Result<Vec<V>>{
    let records = db.select(table).await?;
    let deserialized_records: Vec<V> = records.into_iter().map(|record| {
        serde_json::from_value(record).unwrap()
    }).collect();
    Ok(deserialized_records)
}

//Update
pub async fn reregister<V: serde::Serialize>(db: &mut Db, table: &str, id: &str, new_value: V) -> s::Result<()>{
    db.update((table, id)).content(new_value).await?;
    Ok(())
}


//Delete
pub async fn remove(db: &mut Db, table: &str, id: &str) -> s::Result<()>{
    db.delete((table, id)).await?;
    Ok(())
}

// //Query
// async fn request(db: &mut Db, query: String) -> s::Result<>{
//     db.query(query)
// }