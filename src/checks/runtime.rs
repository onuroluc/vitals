use crate::check::{CheckResult, Status};
use crate::config::VitalsConfig;
use crate::detect::ProjectContext;
use crate::platform::Platform;
use crate::version;

/// Runtime command definitions: (name, [(binary, args)]).
struct RuntimeDef {
    name: &'static str,
    commands: &'static [(&'static str, &'static [&'static str])],
}

const RUNTIMES: &[RuntimeDef] = &[
    RuntimeDef {
        name: "node",
        commands: &[("node", &["--version"])],
    },
    RuntimeDef {
        name: "python",
        commands: &[("python3", &["--version"]), ("python", &["--version"])],
    },
    RuntimeDef {
        name: "rust",
        commands: &[("rustc", &["--version"])],
    },
    RuntimeDef {
        name: "go",
        commands: &[("go", &["version"])],
    },
    RuntimeDef {
        name: "ruby",
        commands: &[("ruby", &["--version"])],
    },
    RuntimeDef {
        name: "java",
        commands: &[("java", &["--version"])],
    },
];

pub fn check(
    ctx: &ProjectContext,
    config: &VitalsConfig,
    platform: &Platform,
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for req in &ctx.runtimes {
        let def = RUNTIMES.iter().find(|d| d.name == req.name);

        // Config overrides auto-detected version requirement.
        let version_req = get_config_req(config, &req.name).or_else(|| req.version_req.clone());

        // Try to get installed version.
        let found_version = def.and_then(|d| {
            for (cmd, args) in d.commands {
                if let Some(output) = version::run_cmd(cmd, args) {
                    if let Some(ver) = version::extract_version(&output) {
                        return Some(ver);
                    }
                }
            }
            None
        });

        match found_version {
            Some(ver) => {
                if let Some(ref vr) = version_req {
                    if version::meets_requirement(&ver, vr) {
                        results.push(CheckResult {
                            category: "Runtime".into(),
                            name: req.name.clone(),
                            status: Status::Pass,
                            found: format!("v{}", ver),
                            expected: format!("{} ({})", vr, req.source),
                            fix: None,
                            details: vec![],
                        });
                    } else {
                        results.push(CheckResult {
                            category: "Runtime".into(),
                            name: req.name.clone(),
                            status: Status::Fail,
                            found: format!("v{}", ver),
                            expected: format!("{} ({})", vr, req.source),
                            fix: Some(upgrade_hint(&req.name, vr)),
                            details: vec![],
                        });
                    }
                } else {
                    // No version requirement — just check if installed.
                    results.push(CheckResult {
                        category: "Runtime".into(),
                        name: req.name.clone(),
                        status: Status::Pass,
                        found: format!("v{}", ver),
                        expected: "installed".into(),
                        fix: None,
                        details: vec![],
                    });
                }
            }
            None => {
                let expected = if let Some(ref vr) = version_req {
                    format!("{} ({})", vr, req.source)
                } else {
                    "installed".into()
                };
                results.push(CheckResult {
                    category: "Runtime".into(),
                    name: req.name.clone(),
                    status: Status::Fail,
                    found: "not found".into(),
                    expected,
                    fix: Some(platform.install_hint(&req.name)),
                    details: vec![],
                });
            }
        }
    }

    results
}

fn get_config_req(config: &VitalsConfig, name: &str) -> Option<String> {
    match name {
        "node" => config.require.node.clone(),
        "python" => config.require.python.clone(),
        "rust" => config.require.rust.clone(),
        "go" => config.require.go.clone(),
        "ruby" => config.require.ruby.clone(),
        "java" => config.require.java.clone(),
        _ => None,
    }
}

fn upgrade_hint(name: &str, required: &str) -> String {
    match name {
        "node" => format!("nvm install {} (or update Node.js)", required),
        "python" => format!("pyenv install {} (or update Python)", required),
        "rust" => "rustup update stable".into(),
        "go" => format!("update Go to {}", required),
        _ => format!("update {} to {}", name, required),
    }
}
