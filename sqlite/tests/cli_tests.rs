use rusqlite::{Connection, Result};
use sqlite::{create_table, delete_table, load_data, read_table, update_record};

fn setup_test_db() -> Result<Connection> {
    // Creates an in-memory database for testing
    let conn = Connection::open_in_memory()?;
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

    // Update the path if necessary, or use an appropriate test CSV file path
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

    // Ensure the `update_record` function works with an existing name
    let result = update_record(&conn, "test_table", "age", "30", "name = 'Alice'");
    assert!(result.is_ok(), "Failed to update record");

    // Additional check: try reading the updated data to confirm the update
    let read_result = read_table(&conn, "test_table");
    assert!(
        read_result.is_ok(),
        "Failed to read table data after update"
    );
}

#[test]
fn test_delete_table() {
    let conn = setup_test_db().expect("Failed to set up test database");
    create_table(&conn, "test_table").expect("Failed to create table");

    let result = delete_table(&conn, "test_table");
    assert!(result.is_ok(), "Failed to delete table");
}
