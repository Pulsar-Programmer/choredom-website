use serde::Serialize;
use serde_json::Value;
use surrealdb as s;
use s::Surreal;
use s::opt::auth::{Root, Scope};
use s::engine::remote::ws::{Client, Ws};

pub type Db = Surreal<Client>;

async fn setup_users(db: &mut Db) -> s::Result<()> {


    //this function will be called from the setup_db function
    todo!()
    // Ok(db)
}


async fn setup_tables(db: &mut Db) -> s::Result<()>{


    // let _ = query_value(db, surrealql, parameters);


    todo!()
}

pub async fn setup_db(db_addr: String) -> s::Result<Db>{
    //Change this into the embedded version when ready for non-data persistence
    let mut db = Surreal::new::<Ws>(db_addr).await?;

    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    

    //config namespace and database
    db.use_ns("choredom").use_db("main").await?;

    // db.authenticate(jwt).await?;

    setup_tables(&mut db);

    Ok(db)
}



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

// async fn create<T: Serialize>(db: &mut Db, base_query: &str, content: T, fields_skip: &[&str]){

    
//     let fields: HashMap<String, String> = serde_yaml::from_value(serde_yaml::to_value(&content).unwrap()).unwrap();
//     for (field, value) in fields{
//         // let plusq = format!("{}")
//     }


    
//     // let parameters = 
//     // query(db, query, parameters)


// }


pub async fn query_value(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Vec<s::Result<Vec<Value>>>>{
    query_all(db, surrealql, parameters).await
}


pub async fn query_all<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Vec<s::Result<Vec<T>>>>{
    let mut result = db.query(surrealql).bind(parameters).await?;
    let mut vec: Vec<Result<Vec<T>, _>> = Vec::new();
    for i in 0..result.num_statements(){
        let result: Result<Vec<T>, _> = result.take(i);
        // println!("{result:?}");
        vec.push(result)
    }
    Ok(vec)
}

//make this something used more frequently for querying without a wanted response.
///Querying without a processed response.
pub async fn sole_query(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<s::Response>{
    db.query(surrealql).bind(parameters).await
}

///Only to get the first part of the result of the query.
pub async fn query_once<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Vec<T>>{
    let mut result = db.query(surrealql).bind(parameters).await?;
    let result: Result<Vec<T>, _> = result.take(0);
    result
}


// pub async fn query_n<T: std::fmt::Debug + serde::de::Deserialize
// ^ no point in making this

// async fn query__wrapper(s: s::Result<Vec<s::Result<Vec<Value>>>>) -> Value{
//     todo!()
// }


//Stabilize these features ASAP
// Extract looks better for error handling these separately and getting an owned copy hoenstly but ig the latter should be sued sicne it alwaus could be used how this oen is.
pub fn extract_first<T>(mut vec: Vec<T>) -> Option<T>{
    if vec.is_empty(){
        return None;
    }
    Some(vec.remove(0))
}

pub async fn query_once_option<T: std::fmt::Debug + serde::de::DeserializeOwned>(db: &mut Db, surrealql: &str, parameters: impl Serialize) -> s::Result<Option<T>>{
    let mut result = db.query(surrealql).bind(parameters).await?;
    let result: Result<Option<T>, _> = result.take(0);
    result
}