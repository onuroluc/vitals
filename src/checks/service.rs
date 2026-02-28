use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::check::{CheckResult, Status};
use crate::config::VitalsConfig;
use crate::detect::{ProjectContext, ServiceReq};
use crate::platform::Platform;
use crate::version;

pub fn check(ctx: &ProjectContext, config: &VitalsConfig, platform: &Platform) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // ── Docker ──────────────────────────────────────────────────────────
    if ctx.has_docker || config.services.docker.unwrap_or(false) {
        let docker_running = version::run_cmd("docker", &["info"]).is_some();
        if docker_running {
            let ver = version::run_cmd("docker", &["--version"])
                .and_then(|o| version::extract_version(&o))
                .unwrap_or_else(|| "unknown".into());
            results.push(CheckResult {
                category: "Services".into(),
                name: "docker".into(),
                status: Status::Pass,
                found: format!("running (v{})", ver),
                expected: "running".into(),
                fix: None,
                details: vec![],
            });
        } else {
            let installed = version::run_cmd("docker", &["--version"]).is_some();
            if installed {
                results.push(CheckResult {
                    category: "Services".into(),
                    name: "docker".into(),
                    status: Status::Fail,
                    found: "not running".into(),
                    expected: "running".into(),
                    fix: Some("open -a Docker (macOS) or sudo systemctl start docker".into()),
                    details: vec![],
                });
            } else {
                results.push(CheckResult {
                    category: "Services".into(),
                    name: "docker".into(),
                    status: Status::Fail,
                    found: "not installed".into(),
                    expected: "installed & running".into(),
                    fix: Some(platform.install_hint("docker")),
                    details: vec![],
                });
            }
        }
    }

    // ── Individual services (docker-compose + config) ───────────────────
    let mut service_reqs: Vec<ServiceReq> = ctx.services.clone();

    // Merge config-defined services.
    merge_config_service(&mut service_reqs, &config.services.redis, "redis", 6379);
    merge_config_service(
        &mut service_reqs,
        &config.services.postgres,
        "postgres",
        5432,
    );
    merge_config_service(&mut service_reqs, &config.services.mysql, "mysql", 3306);
    merge_config_service(&mut service_reqs, &config.services.mongo, "mongo", 27017);

    for svc in &service_reqs {
        let reachable = is_reachable(&svc.host, svc.port);
        if reachable {
            results.push(CheckResult {
                category: "Services".into(),
                name: svc.name.clone(),
                status: Status::Pass,
                found: format!("reachable on :{}", svc.port),
                expected: format!("reachable on :{}", svc.port),
                fix: None,
                details: vec![],
            });
        } else {
            let fix = if ctx.has_docker {
                format!("docker compose up -d {}", svc.name)
            } else {
                platform.service_hint(&svc.name)
            };
            results.push(CheckResult {
                category: "Services".into(),
                name: svc.name.clone(),
                status: Status::Fail,
                found: format!("not reachable on :{}", svc.port),
                expected: format!("reachable on :{}", svc.port),
                fix: Some(fix),
                details: vec![],
            });
        }
    }

    results
}

fn merge_config_service(
    reqs: &mut Vec<ServiceReq>,
    detail: &Option<crate::config::ServiceDetail>,
    name: &str,
    default_port: u16,
) {
    if let Some(d) = detail {
        if !reqs.iter().any(|s| s.name == name) {
            reqs.push(ServiceReq {
                name: name.into(),
                host: d.host.clone().unwrap_or_else(|| "localhost".into()),
                port: d.port.unwrap_or(default_port),
            });
        }
    }
}

fn is_reachable(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(addr) = addrs.next() {
            return TcpStream::connect_timeout(&addr, Duration::from_secs(2)).is_ok();
        }
    }
    false
}
