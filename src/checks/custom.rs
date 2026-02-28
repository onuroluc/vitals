use std::process::Command;

use crate::check::{CheckResult, Status};
use crate::config::VitalsConfig;
use crate::detect::ProjectContext;

pub fn check(ctx: &ProjectContext, config: &VitalsConfig) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for cmd in &config.commands {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&cmd.run)
            .current_dir(&ctx.dir)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                results.push(CheckResult {
                    category: "Commands".into(),
                    name: cmd.name.clone(),
                    status: Status::Pass,
                    found: "passed".into(),
                    expected: "exit 0".into(),
                    fix: None,
                    details: vec![],
                });
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let detail = if stderr.is_empty() {
                    String::from_utf8_lossy(&out.stdout).trim().to_string()
                } else {
                    stderr
                };
                results.push(CheckResult {
                    category: "Commands".into(),
                    name: cmd.name.clone(),
                    status: Status::Fail,
                    found: format!("exit {}", out.status.code().unwrap_or(-1)),
                    expected: "exit 0".into(),
                    fix: Some(format!("run: {}", cmd.run)),
                    details: if detail.is_empty() {
                        vec![]
                    } else {
                        detail
                            .lines()
                            .take(3)
                            .map(|l| l.to_string())
                            .collect()
                    },
                });
            }
            Err(_) => {
                results.push(CheckResult {
                    category: "Commands".into(),
                    name: cmd.name.clone(),
                    status: Status::Fail,
                    found: "failed to execute".into(),
                    expected: "exit 0".into(),
                    fix: Some(format!("check command: {}", cmd.run)),
                    details: vec![],
                });
            }
        }
    }

    results
}
