use std::env;
use rusqlite::{Connection, Result};


fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
    }
}

fn run() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("Transactions.db")?;  

    println!("Creating table...");
    DBFunc::create(&conn)?;
        
    println!("Inserting data...");
    DBFunc::insert(&conn)?;
    
    println!("Querying data...");
    DBFunc::query(&conn)?;

    println!("Updating data...");
    DBFunc::update(&conn)?;

    println!("Querying data...");
    DBFunc::query(&conn)?;

    println!("Deleting data...");
    DBFunc::delete(&conn)?;

    println!("Querying data...");
    DBFunc::query(&conn)?;    
    
    DBFunc::drop(&conn)?;    

    Ok(())
}
