use rusqlite::{params, Connection, Result};

pub fn create_encrypted_db(path: &str, key: &str) -> Result<()> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "key", key)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)",
        [],
    )?;
    conn.execute("INSERT INTO users (name) VALUES (?1)", params!["Alice"])?;
    Ok(())
}

pub fn read_encrypted_db(path: &str, key: &str) -> Result<Vec<String>> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "key", key)?;

    let mut stmt = conn.prepare("SELECT name FROM users")?;
    let names = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypted_sqlite_db() {
        let db_path = "test.db";

        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).expect("Failed to remove existing test DB");
        }

        let key = "secret123";

        // Create a new encrypted database
        create_encrypted_db(db_path, key).expect("Failed to create encrypted DB");

        // Read the encrypted database. (should succeed)
        let result = read_encrypted_db(db_path, key);
        assert!(result.is_ok());
        let names = result.unwrap();
        assert_eq!(names, vec!["Alice"]);

        // If we try to read the database with a wrong password, it should fail.
        let wrong = read_encrypted_db(db_path, "wrongpassword");
        assert!(wrong.is_err());

        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).expect("Failed to remove existing test DB");
        }
    }
}
