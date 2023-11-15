// lib.rs
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Customer {
    cust_id: String,
    name: String,
    sex: String,
}

pub fn create(conn: &Connection) -> Result<()> {
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Customer (
            cust_id TEXT NOT NULL,
            name TEXT NOT NULL,
            sex TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(())
}

pub fn insert(conn: &Connection) -> Result<()> {
    conn.execute( "INSERT INTO Customer (cust_id, name, sex)
              VALUES (?1, ?2, ?3)", params!["001", "John", "Male"])?;
    conn.execute( "INSERT INTO Customer (cust_id, name, sex)
              VALUES (?1, ?2, ?3)", params!["002", "Devin", "Female"])?;              
    conn.execute( "INSERT INTO Customer (cust_id, name, sex)
              VALUES (?1, ?2, ?3)", params!["003", "Sharon", "Female"])?;                  
    Ok(())                        
            
}

pub fn query(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM Customer")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?
        ))
    })?;
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

pub fn update(conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE Customer SET sex = 'Unknown' WHERE cust_id = '001'",
        [],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM Customer WHERE cust_id = '001' ",
        [],
    )?;
    Ok(())
}

pub fn drop(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS Customer", [])?;
    Ok(())
}

mod tests {
    use super::*;
    use rusqlite::Result;

    #[test]
    fn test_create() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create(&conn)?;
        let table_names: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        assert_eq!(table_names, vec!["Customer"]);
        Ok(())
    }

    #[test]
    fn test_insert() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create(&conn)?;
        insert(&conn)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM Customer", [], |row| row.get(0))?;
        assert_eq!(count, 3);
        Ok(())       
    }

    #[test]
    fn test_update() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create(&conn)?;
        insert(&conn)?;
        update(&conn)?;
        let val: String = conn.query_row("SELECT sex FROM Customer WHERE cust_id = '001'", [], |row| row.get(0))?;
        assert_eq!(val, "Unknown");
        Ok(())
    }
    #[test]
    fn test_delete() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create(&conn)?;
        insert(&conn)?;
        update(&conn)?;
        delete(&conn)?;
        let id: String = conn.query_row("SELECT cust_id FROM Customer LIMIT 1", [], |row| row.get(0))?;
        assert_eq!(id, "002");
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM Customer", [], |row| row.get(0))?;
        assert_eq!(count, 2);        
        Ok(())
    }


}
