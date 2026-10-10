//! Safe SQLite reader layer with temporary copy handling to avoid file lock contention.

use crate::errors::ExtractError;
use rusqlite::{Connection, OpenFlags, Row};
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

/// RAII wrapper that queries an isolated copy of a browser's SQLite database.
pub struct SafeSqliteReader {
    _temp_file: NamedTempFile,
    conn: Connection,
}

impl SafeSqliteReader {
    /// Copies the target SQLite file (and companion -wal/-shm files if present)
    /// to a temporary file, then opens a read-only SQLite connection.
    pub fn open_copy_at(source_path: &Path) -> Result<Self, ExtractError> {
        if !source_path.exists() {
            log::debug!("Database file does not exist at {:?}", source_path);
            return Err(ExtractError::DatabaseNotFound(source_path.to_path_buf()));
        }

        let temp_file = NamedTempFile::new()
            .map_err(|e| ExtractError::DatabaseError(format!("Failed to create temp db: {}", e)))?;

        log::debug!(
            "Copying SQLite database from {:?} to temporary location {:?}",
            source_path,
            temp_file.path()
        );

        // Copy primary database file
        fs::copy(source_path, temp_file.path()).map_err(|e| {
            ExtractError::DatabaseError(format!(
                "Failed to copy SQLite database from {:?}: {}",
                source_path, e
            ))
        })?;

        // Also copy companion WAL / SHM files if they exist to capture uncommitted/active transactions
        let wal_source = source_path.with_extension("sqlite-wal");
        if wal_source.exists() {
            log::debug!("Copying companion WAL file {:?}", wal_source);
            let _ = fs::copy(&wal_source, temp_file.path().with_extension("sqlite-wal"));
        }

        let shm_source = source_path.with_extension("sqlite-shm");
        if shm_source.exists() {
            log::debug!("Copying companion SHM file {:?}", shm_source);
            let _ = fs::copy(&shm_source, temp_file.path().with_extension("sqlite-shm"));
        }

        let conn = Connection::open_with_flags(
            temp_file.path(),
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
        )
        .map_err(|e| {
            ExtractError::DatabaseError(format!("Failed to open SQLite connection: {}", e))
        })?;

        log::debug!("Successfully connected to temporary SQLite database copy");

        Ok(Self {
            _temp_file: temp_file,
            conn,
        })
    }

    /// Prepares and executes a SQL query, mapping each row with the provided closure.
    pub fn query<T, F>(&self, sql: &str, mut map_fn: F) -> Result<Vec<T>, ExtractError>
    where
        F: FnMut(&Row) -> rusqlite::Result<T>,
    {
        log::debug!("Executing SQLite query: {}", sql);
        let mut stmt = self
            .conn
            .prepare(sql)
            .map_err(|e| ExtractError::DatabaseError(format!("Failed to prepare SQL: {}", e)))?;

        let rows = stmt
            .query_map([], |row| map_fn(row))
            .map_err(|e| ExtractError::DatabaseError(format!("Query execution failed: {}", e)))?;

        let mut results = Vec::new();
        for row in rows {
            let item = row.map_err(|e| {
                ExtractError::DatabaseError(format!("Failed to read SQLite row: {}", e))
            })?;
            results.push(item);
        }
        log::debug!("SQLite query returned {} rows", results.len());
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::NamedTempFile;

    #[test]
    fn copies_and_queries_sqlite_db_safely() {
        let temp_src = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_src.path()).unwrap();
        conn.execute(
            "CREATE TABLE test_cookies (name TEXT, value TEXT, host TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO test_cookies (name, value, host) VALUES ('test_sess', '123', '.leetcode.com')",
            [],
        )
        .unwrap();
        drop(conn);

        let reader = SafeSqliteReader::open_copy_at(temp_src.path()).expect("Failed to open copy");
        let results = reader
            .query(
                "SELECT name, value FROM test_cookies WHERE host LIKE '%leetcode.com'",
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .expect("Query failed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "test_sess");
        assert_eq!(results[0].1, "123");
    }

    #[test]
    fn returns_error_on_nonexistent_path() {
        let path = Path::new("/nonexistent/path/to/cookies.sqlite");
        let result = SafeSqliteReader::open_copy_at(path);
        assert!(matches!(result, Err(ExtractError::DatabaseNotFound(_))));
    }
}
