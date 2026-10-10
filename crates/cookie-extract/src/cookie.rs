//! Domain models for extracted cookies and supported browsers.

/// Represents a single extracted browser cookie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires_at: Option<i64>,
}

/// Supported web browser targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Browser {
    Firefox,
    Chrome,
    Brave,
    Edge,
    Arc,
    AutoDetect,
}

impl Browser {
    /// Human-readable display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Browser::Firefox => "Firefox",
            Browser::Chrome => "Chrome",
            Browser::Brave => "Brave",
            Browser::Edge => "Edge",
            Browser::Arc => "Arc",
            Browser::AutoDetect => "AutoDetect",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_equality_and_construction() {
        let cookie = Cookie {
            name: "LEETCODE_SESSION".into(),
            value: "xyz123".into(),
            domain: ".leetcode.com".into(),
            path: "/".into(),
            expires_at: Some(1700000000),
        };
        assert_eq!(cookie.name, "LEETCODE_SESSION");
        assert_eq!(cookie.value, "xyz123");
        assert_eq!(cookie.domain, ".leetcode.com");
    }

    #[test]
    fn browser_as_str() {
        assert_eq!(Browser::Firefox.as_str(), "Firefox");
        assert_eq!(Browser::Chrome.as_str(), "Chrome");
        assert_eq!(Browser::Brave.as_str(), "Brave");
    }
}
