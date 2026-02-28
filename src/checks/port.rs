use std::net::TcpListener;

use crate::check::{CheckResult, Status};
use crate::config::VitalsConfig;
use crate::detect::ProjectContext;
use crate::version;

pub fn check(ctx: &ProjectContext, config: &VitalsConfig) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let mut ports_to_check: Vec<u16> = config.ports.check.clone();

    // Deduplicate.
    ports_to_check.sort();
    ports_to_check.dedup();

    // Skip ports already tested by service checks.
    let service_ports: Vec<u16> = ctx.services.iter().map(|s| s.port).collect();
    ports_to_check.retain(|p| !service_ports.contains(p));

    for port in &ports_to_check {
        if is_port_available(*port) {
            results.push(CheckResult {
                category: "Ports".into(),
                name: format!(":{}", port),
                status: Status::Pass,
                found: "available".into(),
                expected: "available".into(),
                fix: None,
                details: vec![],
            });
        } else {
            let process = find_process_on_port(*port);
            let found = match &process {
                Some(proc) => format!("in use by {}", proc),
                None => "in use".into(),
            };
            let fix = process
                .as_ref()
                .and_then(|p| {
                    p.split("PID ")
                        .nth(1)
                        .and_then(|s| s.split(')').next())
                        .map(|pid| format!("kill {}", pid))
                })
                .or_else(|| Some(format!("lsof -i :{} to find the process", port)));
            results.push(CheckResult {
                category: "Ports".into(),
                name: format!(":{}", port),
                status: Status::Fail,
                found,
                expected: "available".into(),
                fix,
                details: vec![],
            });
        }
    }

    results
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

fn find_process_on_port(port: u16) -> Option<String> {
    let output = version::run_cmd(
        "lsof",
        &["-i", &format!(":{}", port), "-sTCP:LISTEN", "-P", "-n"],
    )?;
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            return Some(format!("{} (PID {})", parts[0], parts[1]));
        }
    }
    None
}
