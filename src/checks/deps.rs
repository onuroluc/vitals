use crate::check::{CheckResult, Status};
use crate::detect::ProjectContext;

pub fn check(ctx: &ProjectContext) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for dep in &ctx.deps {
        if dep.exists {
            results.push(CheckResult {
                category: "Dependencies".into(),
                name: dep.name.clone(),
                status: Status::Pass,
                found: "installed".into(),
                expected: "installed".into(),
                fix: None,
                details: vec![],
            });
        } else {
            results.push(CheckResult {
                category: "Dependencies".into(),
                name: dep.name.clone(),
                status: Status::Fail,
                found: "missing".into(),
                expected: "installed".into(),
                fix: Some(dep.install_cmd.clone()),
                details: vec![],
            });
        }
    }

    results
}
