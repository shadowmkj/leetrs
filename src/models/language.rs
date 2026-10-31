//! Language enum and associated helpers.
use clap::ValueEnum;

/// Supported submission languages.
///
/// The `ValueEnum` derive lets Clap accept these directly as CLI arguments.
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Language {
    Python,
    Rust,
    Pandas,
    Mysql,
    Postgres,
}

impl From<&String> for Language {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "python3" => Self::Python,
            "rust" => Self::Rust,
            "pythondata" => Self::Pandas,
            "mysql" => Self::Mysql,
            "postgresql" => Self::Postgres,
            _ => Self::Mysql,
        }
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "python3" => Self::Python,
            "rust" => Self::Rust,
            "pythondata" => Self::Pandas,
            "mysql" => Self::Mysql,
            "postgresql" => Self::Postgres,
            _ => Self::Mysql,
        }
    }
}

impl Language {
    /// Maps a [`Language`] variant to LeetCode's internal language slug string.
    pub fn to_lang_slug(&self) -> &'static str {
        match self {
            Language::Python => "python3",
            Language::Rust => "rust",
            Language::Mysql => "mysql",
            Language::Pandas => "pythondata",
            Language::Postgres => "postgresql",
        }
    }

    /// Infers the language from a file extension. Falls back to MySQL for unknown extensions.
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "py" => Language::Python,
            "rs" => Language::Rust,
            "sql" => Language::Mysql,
            _ => Language::Mysql,
        }
    }

    /// Returns the file extension used for solution files in this language.
    pub fn code_extension(&self) -> &'static str {
        match self {
            Language::Python | Language::Pandas => "py",
            Language::Rust => "rs",
            Language::Mysql | Language::Postgres => "sql",
        }
    }

    /// Returns the single-line comment prefix used in this language.
    ///
    /// Used to write the metadata header at the top of generated code files,
    /// e.g. `# id=1 slug=two-sum lang=python3`.
    pub fn meta_comment_prefix(&self) -> &'static str {
        match self {
            Language::Python | Language::Pandas | Language::Mysql => "#",
            Language::Rust => "//",
            Language::Postgres => "--",
        }
    }
}

/// A problem identifier supplied on the command line — either a numeric ID or a slug.
#[derive(Debug, Clone)]
pub enum Identifier {
    Number(u64),
    String(String),
}
