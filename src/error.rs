//! Central error type for the leetrs engine.
use thiserror::Error;

/// All errors that can be produced by the leetrs engine.
///
/// `EngineError` is the single error type propagated through most of the
/// library. It is thin wrapper around the underlying cause so callers can
/// pattern-match on the variant without depending on the concrete error types
/// from `reqwest` or `serde_json`.
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Authentication error: Missing session token or CSRF token")]
    Auth,
    #[error("Serialization error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("GraphQL error: {0}")]
    GraphQL(String),
    #[error("System error")]
    System,
    #[error("Other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, EngineError>;

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // EngineError: Display / human-readable messages
    // -----------------------------------------------------------------------

    #[test]
    fn error_auth_display() {
        let e = EngineError::Auth;
        assert_eq!(
            e.to_string(),
            "Authentication error: Missing session token or CSRF token"
        );
    }

    #[test]
    fn error_system_display() {
        let e = EngineError::System;
        assert_eq!(e.to_string(), "System error");
    }

    #[test]
    fn error_graphql_display_includes_message() {
        let e = EngineError::GraphQL("something went wrong".to_string());
        let msg = e.to_string();
        assert!(msg.contains("GraphQL error"), "expected 'GraphQL error' in: {msg}");
        assert!(msg.contains("something went wrong"), "expected context in: {msg}");
    }

    #[test]
    fn error_other_display_includes_message() {
        let e = EngineError::Other("custom context".to_string());
        let msg = e.to_string();
        assert!(msg.contains("Other"), "expected 'Other' in: {msg}");
        assert!(msg.contains("custom context"), "expected context in: {msg}");
    }

    // -----------------------------------------------------------------------
    // EngineError: From<serde_json::Error>
    // -----------------------------------------------------------------------

    #[test]
    fn error_from_serde_json_error() {
        // Trigger a real serde_json parse error and convert it via From.
        let serde_err: serde_json::Error =
            serde_json::from_str::<serde_json::Value>("not valid json").unwrap_err();
        let engine_err = EngineError::from(serde_err);
        // Must be the Parse variant, not Other or System.
        assert!(matches!(engine_err, EngineError::Parse(_)));
        let msg = engine_err.to_string();
        assert!(
            msg.contains("Serialization error"),
            "expected 'Serialization error' in: {msg}"
        );
    }

    // -----------------------------------------------------------------------
    // Result<T> type alias behaves correctly
    // -----------------------------------------------------------------------

    #[test]
    fn result_ok_works() {
        let r: Result<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn result_err_is_engine_error() {
        let r: Result<()> = Err(EngineError::System);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "System error");
    }
}
