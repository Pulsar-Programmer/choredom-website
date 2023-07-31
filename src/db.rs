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

pub fn dissolve<T: std::fmt::Debug>(s: s::Result<T>, num: usize){
    match s{
        Ok(t) => println!("Success w/ DB Interaction #{num}: {t:?}"),
        Err(e) => println!("Error w/ DB Interaction #{num}: {e}"),
    }
}

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
async fn create<T: Serialize>(db: &mut Db, table: &str, id: &str, content: T, fields_skip: &[&str]){

    // let mut query = format!("CREATE type::thing({}, $id)", table);
    // let fields: HashMap<String, String> = serde_yaml::from_value(serde_yaml::to_value(&content).unwrap()).unwrap();
    // for (field, value) in fields{

    // }


    
    // let parameters = 
    // query(db, query, parameters)


}




pub async fn query(db: &mut Db, query: &str, parameters: impl Serialize) -> s::Result<String>{
    let result = db.query(query).bind(parameters).await?; //.bind(parameters) might not be correct. Please test this out.
    let string = format!("{result:?}");
    Ok(string)
}

// pub async fn query_select<T: serde::Serialize + serde::de::DeserializeOwned + Default>(db: &mut Db, query: &str, parameters: impl Serialize) -> s::Result<String>{
//     let mut result = db.query(query).bind(parameters).await?; //.bind(parameters) might not be correct. Please test this out.
//     let mut vec: Vec<Result<Vec<T>, _>> = Vec::new();
//     for i in 0..result.num_statements(){

//         let fields: HashMap<String, String> = serde_yaml::from_value(serde_yaml::to_value(&T::default()).unwrap()).unwrap();
//         for (field, _) in fields{
//             vec.push(result.take((i, field.as_str())));
//         }
//     }
//     let string = format!("{result:?}");
//     println!("{string}");
    
//     Ok(string)
// }


async fn query__(db: &mut Db, query: &str, parameters: impl Serialize) -> s::Result<Vec<s::Result<Vec<Value>>>>{
    let mut result = db.query(query).bind(parameters).await?; //.bind(parameters) might not be correct. Please test this out.
    let mut vec: Vec<Result<Vec<Value>, _>> = Vec::new();
    for i in 0..result.num_statements(){
        let result: Result<Vec<Value>, _> = result.take(i);
        vec.push(result)
    }
    Ok(vec)
}

// async fn query__wrapper(s: s::Result<Vec<s::Result<Vec<Value>>>>) -> Value{
//     todo!()
// }