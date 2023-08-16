use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
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

// pub fn dissolve<T: std::fmt::Debug>(s: s::Result<T>, num: usize){
//     match s{
//         Ok(t) => println!("Success w/ DB Interaction #{num}: {t:?}"),
//         Err(e) => println!("Error w/ DB Interaction #{num}: {e}"),
//     }
// }

//Create
// pub async fn register<V: serde::Serialize>(db: &mut Db, table: &str, id: &str, value: V) -> s::Result<()>{
//     db.create((table, id)).content(value).await?;
//     Ok(())
// }

// //Read
// pub async fn retrieve<V: serde::de::DeserializeOwned + std::fmt::Debug>(db: &mut Db, table: &str) -> s::Result<Vec<V>>{
//     let records = db.select(table).await?;
//     let deserialized_records: Vec<V> = records.into_iter().map(|record| {
//         serde_json::from_value(record).unwrap()
//     }).collect();
//     Ok(deserialized_records)
// }

// //Update
// pub async fn reregister<V: serde::Serialize>(db: &mut Db, table: &str, id: &str, new_value: V) -> s::Result<()>{
//     db.update((table, id)).content(new_value).await?;
//     Ok(())
// }


// //Delete
// pub async fn remove(db: &mut Db, table: &str, id: &str) -> s::Result<()>{
//     db.delete((table, id)).await?;
//     Ok(())
// }

// //Query
// async fn request(db: &mut Db, query: &str) -> s::Response{
//     let records = match db.query(query).await{
//         Ok(r) => r,
//         Err(_) => todo!(),
//     };
//     for record in records{
//         let id = &record.id;
//     }
//     todo!()
// }
async fn create<T: Serialize>(db: &mut Db, base_query: &str, content: T, fields_skip: &[&str]){

    
    let fields: HashMap<String, String> = serde_yaml::from_value(serde_yaml::to_value(&content).unwrap()).unwrap();
    for (field, value) in fields{
        // let plusq = format!("{}")
    }


    
    // let parameters = 
    // query(db, query, parameters)


}


pub async fn query_value(db: &mut Db, surrealql: &str, parameters: Option<impl Serialize>) -> s::Result<Vec<s::Result<Vec<Value>>>>{
    query(db, surrealql, parameters).await
}


pub async fn query<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: Option<impl Serialize>) -> s::Result<Vec<s::Result<Vec<T>>>>{
    let mut result = match parameters{
        Some(p) => db.query(surrealql).bind(p).await?,
        None => db.query(surrealql).await?,
    };
    let mut vec: Vec<Result<Vec<T>, _>> = Vec::new();
    for i in 0..result.num_statements(){
        let result: Result<Vec<T>, _> = result.take(i);
        // println!("{result:?}");
        vec.push(result)
    }
    Ok(vec)
}

// async fn query__wrapper(s: s::Result<Vec<s::Result<Vec<Value>>>>) -> Value{
//     todo!()
// }


pub fn transmission_transmit<Args: serde::Serialize>(field: &str, session: &actix_session::Session, args: Args) -> Result<(), actix_session::SessionInsertError>{
    let derived_field = format!("{}_transmitter", field);
    session.insert(derived_field, args)
}
pub fn transmission_receive<Transmitter: serde::de::DeserializeOwned>(field: &str, session: &actix_session::Session) -> Result<Transmitter, Box<dyn std::error::Error>>{
    let derived_field = format!("{}_transmitter", field);
    let value = session.remove(&derived_field).ok_or("Failed to transmit using transmitter.")?;
    Ok(serde_json::from_str(&value)?)
}