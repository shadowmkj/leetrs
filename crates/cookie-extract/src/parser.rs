//! Robust parser for cURL commands, raw `Cookie:` headers, and key-value token strings.

use crate::{cookie::Cookie, errors::ExtractError};

/// Parses raw text input containing a `Cookie:` header, copied cURL command,
/// or delimited key-value pairs (e.g. `LEETCODE_SESSION=...; csrftoken=...`).
pub fn parse_raw_cookie_header(input: &str) -> Result<Vec<Cookie>, ExtractError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ExtractError::InvalidHeaderFormat);
    }

    // Extract cookie body if inside a cURL command (-H 'cookie: ...' or -b '...')
    let cookie_body = extract_cookie_body_from_curl_or_header(trimmed);

    let mut cookies = Vec::new();
    let delimiters = [';', '\n', '&'];
    let tokens: Vec<&str> = cookie_body
        .split(|c| delimiters.contains(&c))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for token in tokens {
        if let Some((key, val)) = token.split_once('=') {
            let name = key.trim().trim_matches('\'').trim_matches('"').to_string();
            let value = val
                .trim()
                .trim_matches('\'')
                .trim_matches('"')
                .trim_end_matches(';')
                .to_string();

            if !name.is_empty() && !value.is_empty() {
                cookies.push(Cookie {
                    name,
                    value,
                    domain: "leetcode.com".to_string(),
                    path: "/".to_string(),
                    expires_at: None,
                });
            }
        }
    }

    if cookies.is_empty() {
        Err(ExtractError::InvalidHeaderFormat)
    } else {
        Ok(cookies)
    }
}

/// Helper to strip cURL prefixes or `Cookie:` / `cookie:` header tags.
fn extract_cookie_body_from_curl_or_header(input: &str) -> &str {
    let lower = input.to_lowercase();

    // Check for cURL header variants: -H 'cookie: ...' or -H "cookie: ..." or -H "Cookie: ..."
    if let Some(idx) = lower.find("-h 'cookie:") {
        let after = &input[idx + 11..];
        after.split('\'').next().unwrap_or(after)
    } else if let Some(idx) = lower.find("-h \"cookie:") {
        let after = &input[idx + 11..];
        after.split('"').next().unwrap_or(after)
    } else if let Some(idx) = lower.find("-b '") {
        let after = &input[idx + 4..];
        after.split('\'').next().unwrap_or(after)
    } else if let Some(idx) = lower.find("-b \"") {
        let after = &input[idx + 4..];
        after.split('"').next().unwrap_or(after)
    } else if let Some(idx) = lower.find("cookie:") {
        &input[idx + 7..]
    } else {
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_standard_cookie_header() {
        let input = "Cookie: LEETCODE_SESSION=sess123; csrftoken=csrf456";
        let cookies = parse_raw_cookie_header(input).expect("Failed to parse");
        assert_eq!(cookies.len(), 2);
        assert_eq!(cookies[0].name, "LEETCODE_SESSION");
        assert_eq!(cookies[0].value, "sess123");
        assert_eq!(cookies[1].name, "csrftoken");
        assert_eq!(cookies[1].value, "csrf456");
    }

    #[test]
    fn parses_curl_command_with_single_quotes() {
        let input = r#"curl 'https://leetcode.com/graphql' -H 'cookie: LEETCODE_SESSION=sess_abc; csrftoken=csrf_def;' --compressed"#;
        let cookies = parse_raw_cookie_header(input).expect("Failed to parse curl");
        assert_eq!(cookies.len(), 2);
        assert_eq!(cookies[0].name, "LEETCODE_SESSION");
        assert_eq!(cookies[0].value, "sess_abc");
        assert_eq!(cookies[1].name, "csrftoken");
        assert_eq!(cookies[1].value, "csrf_def");
    }

    #[test]
    fn parses_curl_command_with_double_quotes() {
        let input = r#"curl "https://leetcode.com/graphql" -H "Cookie: LEETCODE_SESSION=sess_xyz; csrftoken=csrf_uvw""#;
        let cookies = parse_raw_cookie_header(input).expect("Failed to parse curl");
        assert_eq!(cookies.len(), 2);
        assert_eq!(cookies[0].value, "sess_xyz");
        assert_eq!(cookies[1].value, "csrf_uvw");
    }

    #[test]
    fn parses_raw_key_values_separated_by_newlines_or_semicolons() {
        let input = "LEETCODE_SESSION = val1 \n csrftoken = val2";
        let cookies = parse_raw_cookie_header(input).expect("Failed to parse key-values");
        assert_eq!(cookies.len(), 2);
        assert_eq!(cookies[0].value, "val1");
        assert_eq!(cookies[1].value, "val2");
    }

    #[test]
    fn rejects_empty_or_invalid_strings() {
        assert!(parse_raw_cookie_header("").is_err());
        assert!(parse_raw_cookie_header("   ").is_err());
        assert!(parse_raw_cookie_header("just random text without equals").is_err());
    }
}
