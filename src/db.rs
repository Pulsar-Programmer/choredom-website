use serde::{Serialize, Deserialize};
use serde_json::json;
use std::borrow::Cow;
use surrealdb::{Result, Surreal};
use surrealdb::sql;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Scope;
use surrealdb as s;

async fn db() -> s::Result<()>{
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;

    // Create a new person with a random ID
    let created: Person = db.create("person")
        .content(Person {
            title: "Founder & CEO".into(),
            name: Name {
                first: "Tobie".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: true,
        })
        .await?;

    // Create a new person with a specific ID
    let created: Person = db.create(("person", "jaime"))
        .content(Person {
            title: "Founder & COO".into(),
            name: Name {
                first: "Jaime".into(),
                last: "Morgan Hitchcock".into(),
            },
            marketing: false,
        })
        .await?;

    // Update a person record with a specific ID
    let updated: Person = db.update(("person", "jaime"))
        .merge(json!({"marketing": true}))
        .await?;

    // Select all people records
    let people: Vec<Person> = db.select("person").await?;

    // Perform a custom advanced query
    let sql = r#"
        SELECT marketing, count()
        FROM type::table($table)
        GROUP BY marketing
    "#;

    let groups = db.query(sql)
        .bind(("table", "person"))
        .await?;

    Ok(())
}


#[derive(Serialize, Deserialize)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    title: Cow<'static, str>,
    name: Name,
    marketing: bool,
}

#[derive(Serialize)]
struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}


async fn create_users() -> s::Result<Surreal<Client>> {

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

async fn setup_db() -> s::Result<Surreal<Client>>{
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








//Create
async fn register<Value: serde::Serialize>(table: &str, id: &str, value: Value) -> s::Result<()>{
    
    let db = setup_db().await?;
    



    Ok(())
}