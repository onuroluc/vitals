use crate::check::{CheckResult, Status};
use crate::config::VitalsConfig;
use crate::detect::ProjectContext;

pub fn check(ctx: &ProjectContext, config: &VitalsConfig) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Collect expected keys from auto-detect + config.
    let mut expected_keys = ctx.env.expected_keys.clone();
    for key in &config.env.required {
        if !expected_keys.contains(key) {
            expected_keys.push(key.clone());
        }
    }

    if expected_keys.is_empty() {
        return results;
    }

    // Check if .env file exists.
    if ctx.env.env_file.is_none() {
        let example_name = ctx
            .env
            .example_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| ".env.example".into());

        results.push(CheckResult {
            category: "Environment".into(),
            name: ".env".into(),
            status: Status::Fail,
            found: "missing".into(),
            expected: "file exists".into(),
            fix: Some(format!("cp {} .env", example_name)),
            details: vec![],
        });
        return results;
    }

    // Compare keys.
    let missing: Vec<String> = expected_keys
        .iter()
        .filter(|k| !ctx.env.actual_keys.contains(k))
        .cloned()
        .collect();

    if missing.is_empty() {
        results.push(CheckResult {
            category: "Environment".into(),
            name: ".env".into(),
            status: Status::Pass,
            found: format!("{} keys", ctx.env.actual_keys.len()),
            expected: format!("{} required keys", expected_keys.len()),
            fix: None,
            details: vec![],
        });
    } else {
        results.push(CheckResult {
            category: "Environment".into(),
            name: ".env".into(),
            status: Status::Fail,
            found: format!(
                "missing {} key{}",
                missing.len(),
                if missing.len() == 1 { "" } else { "s" }
            ),
            expected: format!("{} required keys", expected_keys.len()),
            fix: Some("add missing keys to .env".into()),
            details: missing.iter().map(|k| format!("{} — missing", k)).collect(),
        });
    }

    results
}
