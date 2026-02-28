use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use regex::Regex;

// ── Detected types ──────────────────────────────────────────────────────────

/// A runtime requirement detected from project files.
#[derive(Debug, Clone)]
pub struct RuntimeReq {
    pub name: String,
    pub version_req: Option<String>,
    pub source: String,
}

/// Dependency directory status.
#[derive(Debug, Clone)]
pub struct DepsInfo {
    pub name: String,
    pub path: PathBuf,
    pub exists: bool,
    pub install_cmd: String,
}

/// Service detected from docker-compose or config.
#[derive(Debug, Clone)]
pub struct ServiceReq {
    pub name: String,
    pub host: String,
    pub port: u16,
}

/// Environment variable context.
#[derive(Debug, Clone)]
pub struct EnvContext {
    pub example_file: Option<PathBuf>,
    pub env_file: Option<PathBuf>,
    pub expected_keys: Vec<String>,
    pub actual_keys: Vec<String>,
}

/// Full project context from auto-detection.
#[derive(Debug, Clone)]
pub struct ProjectContext {
    pub dir: PathBuf,
    pub runtimes: Vec<RuntimeReq>,
    pub deps: Vec<DepsInfo>,
    pub services: Vec<ServiceReq>,
    pub ports: Vec<u16>,
    pub env: EnvContext,
    pub has_docker: bool,
}

// ── Scanner ─────────────────────────────────────────────────────────────────

/// Scan a project directory and auto-detect everything we can.
pub fn scan(dir: &Path) -> Result<ProjectContext> {
    let mut ctx = ProjectContext {
        dir: dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf()),
        runtimes: Vec::new(),
        deps: Vec::new(),
        services: Vec::new(),
        ports: Vec::new(),
        env: EnvContext {
            example_file: None,
            env_file: None,
            expected_keys: Vec::new(),
            actual_keys: Vec::new(),
        },
        has_docker: false,
    };

    detect_node(&mut ctx, dir);
    detect_python(&mut ctx, dir);
    detect_rust(&mut ctx, dir);
    detect_go(&mut ctx, dir);
    detect_ruby(&mut ctx, dir);
    detect_java(&mut ctx, dir);
    detect_docker(&mut ctx, dir);
    detect_env(&mut ctx, dir);

    Ok(ctx)
}

// ── Node.js ─────────────────────────────────────────────────────────────────

fn detect_node(ctx: &mut ProjectContext, dir: &Path) {
    if !dir.join("package.json").exists() {
        return;
    }

    let mut version_req: Option<String> = None;
    let mut source = "package.json".to_string();

    // .nvmrc
    if let Ok(content) = fs::read_to_string(dir.join(".nvmrc")) {
        let v = content.trim().trim_start_matches('v');
        if !v.is_empty() && v.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            version_req = Some(v.to_string());
            source = ".nvmrc".to_string();
        }
    }

    // .node-version
    if version_req.is_none() {
        if let Ok(content) = fs::read_to_string(dir.join(".node-version")) {
            let v = content.trim().trim_start_matches('v');
            if !v.is_empty() && v.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                version_req = Some(v.to_string());
                source = ".node-version".to_string();
            }
        }
    }

    // package.json engines.node
    if version_req.is_none() {
        if let Ok(content) = fs::read_to_string(dir.join("package.json")) {
            if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(node_ver) = pkg
                    .get("engines")
                    .and_then(|e| e.get("node"))
                    .and_then(|v| v.as_str())
                {
                    version_req = Some(node_ver.to_string());
                    source = "package.json engines".to_string();
                }
            }
        }
    }

    ctx.runtimes.push(RuntimeReq {
        name: "node".to_string(),
        version_req,
        source,
    });

    // Detect package manager from lockfiles
    let install_cmd = if dir.join("yarn.lock").exists() {
        "yarn install"
    } else if dir.join("pnpm-lock.yaml").exists() {
        "pnpm install"
    } else if dir.join("bun.lockb").exists() || dir.join("bun.lock").exists() {
        "bun install"
    } else {
        "npm install"
    };

    ctx.deps.push(DepsInfo {
        name: "node_modules".to_string(),
        path: dir.join("node_modules"),
        exists: dir.join("node_modules").exists(),
        install_cmd: install_cmd.to_string(),
    });
}

// ── Python ──────────────────────────────────────────────────────────────────

fn detect_python(ctx: &mut ProjectContext, dir: &Path) {
    let has_project = dir.join("pyproject.toml").exists()
        || dir.join("requirements.txt").exists()
        || dir.join("setup.py").exists()
        || dir.join("Pipfile").exists();

    if !has_project {
        return;
    }

    let mut version_req: Option<String> = None;
    let mut source = "project files".to_string();

    // .python-version
    if let Ok(content) = fs::read_to_string(dir.join(".python-version")) {
        let v = content.trim();
        if !v.is_empty() && v.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            version_req = Some(v.to_string());
            source = ".python-version".to_string();
        }
    }

    // pyproject.toml requires-python
    if version_req.is_none() {
        if let Ok(content) = fs::read_to_string(dir.join("pyproject.toml")) {
            if let Ok(val) = content.parse::<toml::Value>() {
                if let Some(req) = val
                    .get("project")
                    .and_then(|p| p.get("requires-python"))
                    .and_then(|v| v.as_str())
                {
                    version_req = Some(req.to_string());
                    source = "pyproject.toml".to_string();
                }
            }
        }
    }

    ctx.runtimes.push(RuntimeReq {
        name: "python".to_string(),
        version_req,
        source,
    });

    // Detect virtualenv
    let venv_found = [".venv", "venv", "env"].iter().any(|name| {
        let p = dir.join(name);
        p.join("bin/python").exists() || p.join("Scripts/python.exe").exists()
    });

    let install_cmd = if dir.join("Pipfile").exists() {
        "pipenv install"
    } else if dir.join("pyproject.toml").exists() {
        if fs::read_to_string(dir.join("pyproject.toml"))
            .map(|c| c.contains("[tool.poetry]"))
            .unwrap_or(false)
        {
            "poetry install"
        } else {
            "python3 -m venv .venv && pip install -e ."
        }
    } else {
        "python3 -m venv .venv && pip install -r requirements.txt"
    };

    ctx.deps.push(DepsInfo {
        name: "virtualenv".to_string(),
        path: dir.join(".venv"),
        exists: venv_found,
        install_cmd: install_cmd.to_string(),
    });
}

// ── Rust ────────────────────────────────────────────────────────────────────

fn detect_rust(ctx: &mut ProjectContext, dir: &Path) {
    if !dir.join("Cargo.toml").exists() {
        return;
    }

    let mut version_req: Option<String> = None;
    let mut source = "Cargo.toml".to_string();

    // rust-toolchain.toml
    if let Ok(content) = fs::read_to_string(dir.join("rust-toolchain.toml")) {
        if let Ok(val) = content.parse::<toml::Value>() {
            if let Some(channel) = val
                .get("toolchain")
                .and_then(|t| t.get("channel"))
                .and_then(|v| v.as_str())
            {
                if channel.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                    version_req = Some(format!(">={}", channel));
                    source = "rust-toolchain.toml".to_string();
                }
            }
        }
    }

    // Cargo.toml rust-version (MSRV)
    if version_req.is_none() {
        if let Ok(content) = fs::read_to_string(dir.join("Cargo.toml")) {
            if let Ok(val) = content.parse::<toml::Value>() {
                if let Some(rv) = val
                    .get("package")
                    .and_then(|p| p.get("rust-version"))
                    .and_then(|v| v.as_str())
                {
                    version_req = Some(format!(">={}", rv));
                    source = "Cargo.toml rust-version".to_string();
                }
            }
        }
    }

    ctx.runtimes.push(RuntimeReq {
        name: "rust".to_string(),
        version_req,
        source,
    });
}

// ── Go ──────────────────────────────────────────────────────────────────────

fn detect_go(ctx: &mut ProjectContext, dir: &Path) {
    if !dir.join("go.mod").exists() {
        return;
    }

    let mut version_req: Option<String> = None;
    if let Ok(content) = fs::read_to_string(dir.join("go.mod")) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("go ") {
                let ver = line.strip_prefix("go ").unwrap().trim();
                version_req = Some(format!(">={}", ver));
                break;
            }
        }
    }

    ctx.runtimes.push(RuntimeReq {
        name: "go".to_string(),
        version_req,
        source: "go.mod".to_string(),
    });
}

// ── Ruby ────────────────────────────────────────────────────────────────────

fn detect_ruby(ctx: &mut ProjectContext, dir: &Path) {
    if !dir.join("Gemfile").exists() {
        return;
    }

    let mut version_req: Option<String> = None;
    let mut source = "Gemfile".to_string();

    if let Ok(content) = fs::read_to_string(dir.join(".ruby-version")) {
        let v = content.trim();
        if !v.is_empty() {
            version_req = Some(v.to_string());
            source = ".ruby-version".to_string();
        }
    }

    ctx.runtimes.push(RuntimeReq {
        name: "ruby".to_string(),
        version_req,
        source,
    });
}

// ── Java ────────────────────────────────────────────────────────────────────

fn detect_java(ctx: &mut ProjectContext, dir: &Path) {
    let has_maven = dir.join("pom.xml").exists();
    let has_gradle = dir.join("build.gradle").exists() || dir.join("build.gradle.kts").exists();

    if !has_maven && !has_gradle {
        return;
    }

    let source = if has_maven { "pom.xml" } else { "build.gradle" };
    ctx.runtimes.push(RuntimeReq {
        name: "java".to_string(),
        version_req: None,
        source: source.to_string(),
    });
}

// ── Docker / Compose ────────────────────────────────────────────────────────

fn detect_docker(ctx: &mut ProjectContext, dir: &Path) {
    let compose_files = [
        "docker-compose.yml",
        "docker-compose.yaml",
        "compose.yml",
        "compose.yaml",
    ];

    let mut compose_content = None;
    for name in &compose_files {
        if let Ok(content) = fs::read_to_string(dir.join(name)) {
            compose_content = Some(content);
            break;
        }
    }

    if dir.join("Dockerfile").exists() || compose_content.is_some() {
        ctx.has_docker = true;
    }

    if let Some(content) = compose_content {
        parse_compose_services(&mut ctx.services, &mut ctx.ports, &content);
    }
}

fn parse_compose_services(services: &mut Vec<ServiceReq>, ports: &mut Vec<u16>, content: &str) {
    let known: &[(&str, &str, u16)] = &[
        ("redis", "redis", 6379),
        ("postgres", "postgres", 5432),
        ("postgresql", "postgres", 5432),
        ("mysql", "mysql", 3306),
        ("mariadb", "mysql", 3306),
        ("mongo", "mongo", 27017),
        ("mongodb", "mongo", 27017),
        ("rabbitmq", "rabbitmq", 5672),
        ("memcached", "memcached", 11211),
        ("elasticsearch", "elasticsearch", 9200),
        ("minio", "minio", 9000),
        ("mailpit", "mailpit", 1025),
    ];

    let lower = content.to_lowercase();

    for (pattern, canonical, default_port) in known {
        if !lower.contains(pattern) {
            continue;
        }
        // Avoid duplicates (postgres & postgresql both map to "postgres").
        if services.iter().any(|s| s.name == *canonical) {
            continue;
        }

        // Try to find a port mapping like "XXXX:default_port"
        let re_str = format!(r#"["']?(\d+):{}["']?"#, default_port);
        let port = Regex::new(&re_str)
            .ok()
            .and_then(|re| re.captures(content))
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(*default_port);

        services.push(ServiceReq {
            name: canonical.to_string(),
            host: "localhost".to_string(),
            port,
        });
        ports.push(port);
    }
}

// ── Environment files ───────────────────────────────────────────────────────

fn detect_env(ctx: &mut ProjectContext, dir: &Path) {
    let example_files = [".env.example", ".env.sample", ".env.template"];

    for name in &example_files {
        let path = dir.join(name);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                ctx.env.example_file = Some(path);
                ctx.env.expected_keys = parse_env_keys(&content);
            }
            break;
        }
    }

    let env_path = dir.join(".env");
    if env_path.is_file() {
        if let Ok(content) = fs::read_to_string(&env_path) {
            ctx.env.env_file = Some(env_path);
            ctx.env.actual_keys = parse_env_keys(&content);
        }
    }
}

fn parse_env_keys(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| {
            let l = l.strip_prefix("export ").unwrap_or(l);
            l.split('=').next().map(|k| k.trim().to_string())
        })
        .filter(|k| !k.is_empty())
        .collect()
}
