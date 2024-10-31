use rusqlite::{Connection, Result};
use sqlite::{create_table, load_data, read_table, update_record, delete_table};

fn setup_test_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?; // Creates an in-memory database for testing
    Ok(conn)
}

#[test]
fn test_create_table() {
    let conn = setup_test_db().expect("Failed to set up test database");
    let result = create_table(&conn, "test_table");
    assert!(result.is_ok(), "Failed to create table");
}

#[test]
fn test_load_data() {
    let conn = setup_test_db().expect("Failed to set up test database");
    create_table(&conn, "test_table").expect("Failed to create table");

    let result = load_data(&conn, "test_table", "../data/sample_data.csv");
    assert!(result.is_ok(), "Failed to load data into table");
}

#[test]
fn test_read_table() {
    let conn = setup_test_db().expect("Failed to set up test database");
    create_table(&conn, "test_table").expect("Failed to create table");
    load_data(&conn, "test_table", "../data/sample_data.csv").expect("Failed to load data");

    let result = read_table(&conn, "test_table");
    assert!(result.is_ok(), "Failed to read table data");
}

#[test]
fn test_update_record() {
    let conn = setup_test_db().expect("Failed to set up test database");
    create_table(&conn, "test_table").expect("Failed to create table");
    load_data(&conn, "test_table", "../data/sample_data.csv").expect("Failed to load data");

    let result = update_record(&conn, "test_table", "age", "30", "name = 'John'");
    assert!(result.is_ok(), "Failed to update record");
}

#[test]
fn test_delete_table() {
    let conn = setup_test_db().expect("Failed to set up test database");
    cre
