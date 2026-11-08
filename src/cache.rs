//! Centralised disk-cache service for the leetrs engine.
//!
//! All paths under the OS data directory are owned by [`CacheService`].
//! This eliminates the duplicated `ProjectDirs` calls that were scattered
//! across `picker.rs` and `auth.rs`.
use crate::error::{EngineError, Result};
use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

/// Provides typed read/write access to the application's OS data directory.
///
/// All cache files live under the directory returned by
/// `ProjectDirs::from("com", "shadowmkj", "leetrs").data_dir()`.
#[derive(Clone, Debug)]
pub struct CacheService {
    data_dir: PathBuf,
}

impl CacheService {
    /// Creates a new [`CacheService`], initialising the data directory on disk
    /// if it does not already exist.
    pub fn new() -> Self {
        let project_dirs = ProjectDirs::from("com", "shadowmkj", "leetrs")
            .expect("Failed to resolve OS data directory");
        Self::with_dir(project_dirs.data_dir().to_path_buf())
    }

    /// Creates a [`CacheService`] with an explicit data directory path.
    pub fn with_dir(data_dir: PathBuf) -> Self {
        if let Err(e) = fs::create_dir_all(&data_dir) {
            eprintln!("❌ Failed to create data directory: {}", e);
        }
        Self { data_dir }
    }

    /// Returns the full path for a named cache file inside the data directory.
    fn path(&self, name: &str) -> PathBuf {
        self.data_dir.join(name)
    }

    /// Returns the path to `data.json` (the cached problem list).
    pub fn problems_path(&self) -> PathBuf {
        self.path("data.json")
    }

    /// Returns the path to `user.json` (the cached user profile).
    pub fn user_path(&self) -> PathBuf {
        self.path("user.json")
    }

    /// Deserialises a typed value from a named cache file.
    ///
    /// Returns `Ok(None)` if the file does not exist, `Err` on I/O or parse failure.
    pub fn read<T: DeserializeOwned>(&self, name: &str) -> Result<Option<T>> {
        let path = self.path(name);
        match File::open(&path) {
            Ok(file) => {
                let value: T =
                    serde_json::from_reader(BufReader::new(file)).map_err(stream_error)?;
                Ok(Some(value))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(EngineError::Io(e)),
        }
    }

    /// Serialises a value and writes it to a named cache file.
    ///
    /// The file is truncated first, so a failed write can leave it empty or partial.
    pub fn write<T: Serialize>(&self, name: &str, value: &T) -> Result<()> {
        let path = self.path(name);
        let mut writer = BufWriter::new(File::create(&path).map_err(EngineError::Io)?);
        serde_json::to_writer(&mut writer, value).map_err(stream_error)?;
        writer.flush().map_err(EngineError::Io)
    }

    /// Reads raw string content from a named cache file.
    ///
    /// Returns `None` if the file does not exist.
    pub fn read_raw(&self, name: &str) -> Option<String> {
        fs::read_to_string(self.path(name)).ok()
    }

    /// Writes raw string content to a named cache file.
    pub fn write_raw(&self, name: &str, content: &str) -> Result<()> {
        fs::write(self.path(name), content).map_err(EngineError::Io)
    }

    /// Atomically writes raw string content to a named cache file.
    pub fn write_raw_atomic(&self, name: &str, content: &str) -> Result<()> {
        write_atomic(&self.path(name), content.as_bytes()).map_err(EngineError::Io)
    }

    /// Removes cache files (`data.json` and `user.json`) from disk.
    ///
    /// Returns the list of paths that were deleted. Missing files are safely ignored.
    pub fn clean(&self) -> Result<Vec<PathBuf>> {
        let targets = [self.problems_path(), self.user_path()];
        let mut removed = Vec::new();

        for path in targets {
            if path.exists() {
                fs::remove_file(&path).map_err(EngineError::Io)?;
                removed.push(path);
            }
        }

        Ok(removed)
    }
}

/// Maps a streaming serde_json error, keeping I/O failures as [`EngineError::Io`].
fn stream_error(e: serde_json::Error) -> EngineError {
    if e.is_io() {
        EngineError::Io(e.into())
    } else {
        EngineError::Parse(e)
    }
}

/// Atomically writes content to a file by writing to a temporary file in the same
/// directory and renaming it into place.
pub fn write_atomic(target: &std::path::Path, content: &[u8]) -> std::io::Result<()> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);

    if let Some(parent) = target.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let parent = target.parent().unwrap_or_else(|| std::path::Path::new("."));
    let filename = target
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("cache");
    let count = ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let tmp_path = parent.join(format!(
        ".{}.tmp-{}-{}-{}",
        filename,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        count
    ));

    fs::write(&tmp_path, content)?;
    if let Err(e) = fs::rename(&tmp_path, target) {
        let _ = fs::remove_file(&tmp_path);
        return Err(e);
    }
    Ok(())
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) struct TempTestDir(pub PathBuf);

#[cfg(test)]
impl TempTestDir {
    pub fn new(prefix: &str) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "{}-{}-{}-{}",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos(),
            id
        ));
        let _ = fs::create_dir_all(&path);
        Self(path)
    }

    pub fn path(&self) -> &std::path::Path {
        &self.0
    }
}

#[cfg(test)]
impl Drop for TempTestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write_round_trip_through_the_file() {
        let temp_dir = TempTestDir::new("leetrs-cache");
        let cache = CacheService::with_dir(temp_dir.path().to_path_buf());

        assert!(cache.read::<Vec<u32>>("missing.json").unwrap().is_none());

        cache.write("values.json", &vec![1, 2, 3]).unwrap();
        assert_eq!(
            cache.read::<Vec<u32>>("values.json").unwrap(),
            Some(vec![1, 2, 3])
        );

        fs::write(temp_dir.path().join("corrupt.json"), "not json").unwrap();
        assert!(cache.read::<Vec<u32>>("corrupt.json").is_err());

        // Test clean() removes problems_path and user_path
        fs::write(cache.problems_path(), "[]").unwrap();
        fs::write(cache.user_path(), "{}").unwrap();
        assert!(cache.problems_path().exists());
        assert!(cache.user_path().exists());

        let removed = cache.clean().unwrap();
        assert_eq!(removed.len(), 2);
        assert!(!cache.problems_path().exists());
        assert!(!cache.user_path().exists());

        // Second clean() call on already cleaned cache returns empty list without error
        let removed_again = cache.clean().unwrap();
        assert!(removed_again.is_empty());
    }

    #[test]
    fn test_write_atomic_and_write_raw_atomic() {
        let temp_dir = TempTestDir::new("leetrs-atomic-test");
        let cache = CacheService::with_dir(temp_dir.path().to_path_buf());
        let target = temp_dir.path().join("atomic.txt");

        // 1. Initial write
        write_atomic(&target, b"initial content").unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "initial content");

        // 2. Overwrite atomically
        write_atomic(&target, b"updated content").unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "updated content");

        // 3. Test CacheService write_raw_atomic
        cache.write_raw_atomic("test.json", "{\"key\": 1}").unwrap();
        assert_eq!(cache.read_raw("test.json").unwrap(), "{\"key\": 1}");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn write_reports_io_failures_as_io_errors() {
        let cache = CacheService {
            data_dir: "/dev".into(),
        };

        assert!(matches!(cache.write("full", &1), Err(EngineError::Io(_))));
        // Larger than the buffer, so serde_json reports the failure itself.
        assert!(matches!(
            cache.write("full", &vec![0; 100_000]),
            Err(EngineError::Io(_))
        ));
    }
}
