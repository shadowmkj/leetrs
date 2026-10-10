//! Safe SQLite reader layer with temporary copy handling to avoid file lock contention.

use crate::errors::ExtractError;
use rusqlite::{Connection, OpenFlags, Row};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// RAII wrapper that queries an isolated copy of a browser's SQLite database.
pub struct SafeSqliteReader {
    _temp_dir: TempDir,
    conn: Connection,
}

impl SafeSqliteReader {
    /// Copies the target SQLite file (and companion -wal/-shm files if present)
    /// into an isolated temporary directory preserving filenames, then opens a read-only SQLite connection.
    pub fn open_copy_at(source_path: &Path) -> Result<Self, ExtractError> {
        if !source_path.exists() {
            log::debug!("Database file does not exist at {:?}", source_path);
            return Err(ExtractError::DatabaseNotFound(source_path.to_path_buf()));
        }

        let temp_dir = tempfile::tempdir().map_err(|e| {
            ExtractError::DatabaseError(format!("Failed to create temp dir: {}", e))
        })?;

        let file_name = source_path
            .file_name()
            .unwrap_or_else(|| OsStr::new("cookies.sqlite"));
        let dest_db = temp_dir.path().join(file_name);

        log::debug!(
            "Copying SQLite database from {:?} to temporary location {:?}",
            source_path,
            dest_db
        );

        // Copy primary database file
        fs::copy(source_path, &dest_db).map_err(|e| {
            ExtractError::DatabaseError(format!(
                "Failed to copy SQLite database from {:?}: {}",
                source_path, e
            ))
        })?;

        // Also copy companion WAL / SHM files if they exist to capture uncommitted/active transactions.
        // SQLite appends "-wal" and "-shm" directly to the full database file name (e.g. "Cookies-wal", "cookies.sqlite-wal").
        let mut wal_source_os = source_path.as_os_str().to_os_string();
        wal_source_os.push("-wal");
        let wal_source = PathBuf::from(wal_source_os);

        if wal_source.exists() {
            log::debug!("Copying companion WAL file {:?}", wal_source);
            let mut dest_wal_os = dest_db.as_os_str().to_os_string();
            dest_wal_os.push("-wal");
            let _ = fs::copy(&wal_source, PathBuf::from(dest_wal_os));
        }

        let mut shm_source_os = source_path.as_os_str().to_os_string();
        shm_source_os.push("-shm");
        let shm_source = PathBuf::from(shm_source_os);

        if shm_source.exists() {
            log::debug!("Copying companion SHM file {:?}", shm_source);
            let mut dest_shm_os = dest_db.as_os_str().to_os_string();
            dest_shm_os.push("-shm");
            let _ = fs::copy(&shm_source, PathBuf::from(dest_shm_os));
        }

        let conn = Connection::open_with_flags(
            &dest_db,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
        )
        .map_err(|e| {
            ExtractError::DatabaseError(format!("Failed to open SQLite connection: {}", e))
        })?;

        log::debug!("Successfully connected to temporary SQLite database copy");

        Ok(Self {
            _temp_dir: temp_dir,
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

    #[test]
    fn copies_wal_and_shm_files_correctly() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("Cookies"); // Chromium-style basename without .sqlite

        let conn = Connection::open(&db_path).unwrap();
        conn.execute_batch("PRAGMA journal_mode = WAL;").unwrap();
        conn.execute(
            "CREATE TABLE test_cookies (name TEXT, value TEXT, host TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO test_cookies (name, value, host) VALUES ('wal_cookie', 'secret', 'leetcode.com')",
            [],
        )
        .unwrap();

        // Keep conn open so lock/WAL state is live
        let reader = SafeSqliteReader::open_copy_at(&db_path).expect("Failed to stage copy");
        let results = reader
            .query(
                "SELECT name, value FROM test_cookies WHERE host LIKE '%leetcode.com%'",
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .expect("Query failed");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "wal_cookie");
        assert_eq!(results[0].1, "secret");

        drop(reader);
        drop(conn);
    }
}
