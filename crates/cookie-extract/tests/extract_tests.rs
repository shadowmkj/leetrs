//! Integration tests for cookie-extract public API.

use cookie_extract::{Browser, Cookie, parse_raw_cookie_header};

#[test]
fn parser_integration_with_mixed_headers() {
    let raw = "Cookie: LEETCODE_SESSION=test_session_value; csrftoken=test_csrf_token";
    let cookies = parse_raw_cookie_header(raw).expect("Parsing failed");
    assert_eq!(cookies.len(), 2);
    assert_eq!(
        cookies[0],
        Cookie {
            name: "LEETCODE_SESSION".into(),
            value: "test_session_value".into(),
            domain: "leetcode.com".into(),
            path: "/".into(),
            expires_at: None,
        }
    );
    assert_eq!(
        cookies[1],
        Cookie {
            name: "csrftoken".into(),
            value: "test_csrf_token".into(),
            domain: "leetcode.com".into(),
            path: "/".into(),
            expires_at: None,
        }
    );
}

#[test]
fn browser_auto_detect_falls_back_cleanly() {
    let result = cookie_extract::extract(Browser::AutoDetect, &["nonexistent-test-domain.invalid"]);
    assert!(result.is_err());
}

#[test]
fn test_chrome_live_extraction_if_available() {
    if let Ok(cookies) =
        cookie_extract::extract(Browser::Chrome, &["leetcode.com", ".leetcode.com"])
    {
        println!("Extracted {} cookies from Chrome", cookies.len());
        for c in &cookies {
            println!("  Cookie: {} (length = {})", c.name, c.value.len());
            // Ensure no leading garbage characters from header
            assert!(!c.value.starts_with('\0'));
        }
    }
}
