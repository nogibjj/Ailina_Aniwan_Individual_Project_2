use rusqlite::{Connection, Result};
use std::fs::File;
use csv::ReaderBuilder;

pub fn create_table(conn: &Connection, table: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY, name TEXT, age INTEGER, city TEXT)",
        table
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table);
    Ok(())
}

pub fn read_table(conn: &Connection, table: &str) -> Result<()> {
    let query = format!("SELECT * FROM {}", table);
    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| {
        // Safely handle missing data with unwrap_or_default
        let id: i32 = row.get(0).unwrap_or_default();
        let name: String = row.get(1).unwrap_or_default();
        let age: i32 = row.get(2).unwrap_or_default();
        let city: String = row.get(3).unwrap_or_default();
        Ok((id, name, age, city))
    })?;

    for row in rows {
        let (id, name, age, city) = row?;
        println!("ID: {}, Name: {}, Age: {}, City: {}", id, name, age, city);
    }
    Ok(())
}

pub fn update_record(
    conn: &Connection,
    table: &str,
    column: &str,
    new_value: &str,
    condition: &str,
) -> Result<()> {
    let update_query = format!(
        "UPDATE {} SET {} = ?1 WHERE {}",
        table, column, condition
    );
    conn.execute(&update_query, [new_value])?;
    println!("Record in table '{}' updated successfully.", table);
    Ok(())
}

pub fn delete_table(conn: &Connection, table: &str) -> Result<()> {
    let delete_query = format!("DROP TABLE IF EXISTS {}", table);
    conn.execute(&delete_query, [])?;
    println!("Table '{}' dropped successfully.", table);
    Ok(())
}

pub fn load_data(conn: &Connection, table: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new().from_reader(File::open(file_path)?);

    for record in reader.records() {
        let record = record?;
        conn.execute(
            &format!("INSERT INTO {} (name, age, city) VALUES (?1, ?2, ?3)", table),
            &[&record[0], &record[1], &record[2]],
        )?;
    }

    println!("Data loaded successfully into table '{}'.", table);
    Ok(())
}
