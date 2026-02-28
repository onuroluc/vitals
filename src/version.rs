use regex::Regex;
use semver::{Version, VersionReq};
use std::process::Command;

/// Run a command and return its stdout (or stderr) as a trimmed string.
pub fn run_cmd(name: &str, args: &[&str]) -> Option<String> {
    Command::new(name)
        .args(args)
        .output()
        .ok()
        .and_then(|out| {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            if out.status.success() && !stdout.is_empty() {
                Some(stdout)
            } else if !stderr.is_empty() {
                Some(stderr)
            } else if !stdout.is_empty() {
                Some(stdout)
            } else {
                None
            }
        })
}

/// Extract a version string (X.Y.Z or X.Y) from arbitrary command output.
pub fn extract_version(output: &str) -> Option<String> {
    let re = Regex::new(r"(\d+\.\d+(?:\.\d+)?)").ok()?;
    re.captures(output)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Normalize a version string to strict semver (X.Y.Z).
pub fn normalize_version(v: &str) -> String {
    let v = v.trim().trim_start_matches(['v', 'V']);
    let parts: Vec<&str> = v.split('.').collect();
    match parts.len() {
        0 => "0.0.0".to_string(),
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => format!("{}.{}.{}", parts[0], parts[1], parts[2]),
    }
}

/// Normalize a requirement string into a semver VersionReq.
/// "20" → ">=20.0.0, <21.0.0"
/// ">=18" → ">=18.0.0"
/// "20.10" → ">=20.10.0, <20.11.0"
pub fn normalize_requirement(req: &str) -> String {
    let req = req.trim();

    // If it already has an operator, use as-is.
    if req.starts_with(">=")
        || req.starts_with("<=")
        || req.starts_with('>')
        || req.starts_with('<')
        || req.starts_with('=')
        || req.starts_with('^')
        || req.starts_with('~')
        || req.contains(',')
    {
        return req.to_string();
    }

    // Plain version number — interpret as a range.
    let v = req.trim_start_matches(['v', 'V']);
    let parts: Vec<u64> = v.split('.').filter_map(|p| p.parse().ok()).collect();
    match parts.len() {
        0 => ">=0.0.0".to_string(),
        1 => format!(">={}.0.0, <{}.0.0", parts[0], parts[0] + 1),
        2 => format!(
            ">={}.{}.0, <{}.{}.0",
            parts[0],
            parts[1],
            parts[0],
            parts[1] + 1
        ),
        _ => format!(">={}.{}.{}", parts[0], parts[1], parts[2]),
    }
}

/// Check if a found version meets a requirement string.
pub fn meets_requirement(found: &str, required: &str) -> bool {
    let norm_found = normalize_version(found);
    let norm_req = normalize_requirement(required);
    if let (Ok(version), Ok(req)) = (Version::parse(&norm_found), VersionReq::parse(&norm_req)) {
        return req.matches(&version);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        assert_eq!(extract_version("v20.10.0"), Some("20.10.0".into()));
        assert_eq!(extract_version("Python 3.12.1"), Some("3.12.1".into()));
        assert_eq!(
            extract_version("rustc 1.75.0 (abcd 2024)"),
            Some("1.75.0".into())
        );
        assert_eq!(
            extract_version("go version go1.21.5 darwin/arm64"),
            Some("1.21.5".into())
        );
    }

    #[test]
    fn test_normalize_version() {
        assert_eq!(normalize_version("20"), "20.0.0");
        assert_eq!(normalize_version("v20.10"), "20.10.0");
        assert_eq!(normalize_version("3.12.1"), "3.12.1");
    }

    #[test]
    fn test_meets_requirement() {
        assert!(meets_requirement("20.10.0", ">=18"));
        assert!(!meets_requirement("16.0.0", ">=18"));
        assert!(meets_requirement("3.12.1", "3.12"));
        assert!(!meets_requirement("3.11.0", "3.12"));
        assert!(meets_requirement("1.75.0", ">=1.70"));
        assert!(meets_requirement("20.10.0", "20"));
        assert!(!meets_requirement("21.0.0", "20"));
    }
}
