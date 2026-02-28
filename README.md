<div align="center">

# vitals

**Universal development environment doctor**

Auto-detects your project stack and diagnoses issues before they waste your time

[![CI](https://github.com/onuroluc/vitals/actions/workflows/ci.yml/badge.svg)](https://github.com/onuroluc/vitals/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)

</div>

---

## The Problem

You clone a repo. You run it. It doesn't work.

- Wrong Node version. Wrong Python version.
- `node_modules` not installed.
- Docker not running. Redis not reachable.  
- `.env` file missing `DATABASE_URL`.
- Port 3000 already in use.

You spend 30 minutes debugging before writing a single line of code.

## The Fix

```bash
$ cd my-project
$ vitals
```

```
  vitals — project health check
  ══════════════════════════════

  Runtime
  ✓ node           v20.10.0                >=18 (.nvmrc)
  ✗ python          not found               → brew install python@3

  Dependencies
  ✓ node_modules    installed

  Services
  ✓ docker          running (v24.0.7)
  ✗ redis           not reachable on :6379  → docker compose up -d redis
  ✓ postgres        reachable on :5432

  Ports
  ✗ :3000           in use by node (PID 48201)  → kill 48201

  Environment
  ✗ .env            missing 2 keys          → add missing keys to .env
    DATABASE_URL — missing
    REDIS_URL — missing

  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  4/8 checks passed — 4 issues found
```

Every failure tells you **exactly** what to run to fix it.

## Features

- **Zero config** — auto-detects `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, `docker-compose.yml`, `.env.example`, and more
- **6 ecosystems** — Node.js, Python, Rust, Go, Ruby, Java
- **Version checking** — reads `.nvmrc`, `.python-version`, `rust-toolchain.toml`, `go.mod`, `package.json engines`
- **Dependency status** — checks `node_modules`, virtualenvs, lockfile presence
- **Service reachability** — Docker, Redis, PostgreSQL, MySQL, MongoDB, RabbitMQ
- **Port availability** — detects what process is blocking a port
- **Env file diffing** — compares `.env` vs `.env.example` for missing keys
- **Custom commands** — run any shell command as a health check
- **CI mode** — `vitals --ci` exits non-zero on failures (use in pipelines)
- **Smart fix suggestions** — tailored to macOS (brew), Debian (apt), Fedora (dnf), Arch (pacman)
- **Single binary** — no runtime dependencies, fast startup
- **Team-shareable** — commit `.vitals.toml` so the whole team benefits

## Installation

### Quick install (recommended)

```bash
curl -sSfL https://raw.githubusercontent.com/onuroluc/vitals/main/install.sh | sh
```

### Homebrew (macOS & Linux)

```bash
brew tap onuroluc/tap
brew install vitals
```

### Debian / Ubuntu (.deb)

```bash
# Download the latest .deb (amd64)
curl -sSLO https://github.com/onuroluc/vitals/releases/latest/download/vitals_0.1.0_amd64.deb
sudo dpkg -i vitals_0.1.0_amd64.deb
```

### Fedora / RHEL / CentOS (.rpm)

```bash
# Download the latest .rpm (x86_64)
curl -sSLO https://github.com/onuroluc/vitals/releases/latest/download/vitals-0.1.0-1.x86_64.rpm
sudo rpm -i vitals-0.1.0-1.x86_64.rpm
```

### Cargo (from crates.io)

```bash
cargo install vitals
```

### From source

```bash
git clone https://github.com/onuroluc/vitals.git
cd vitals
cargo install --path .
```

### From GitHub releases (manual)

```bash
# macOS ARM (Apple Silicon)
curl -sSfL https://github.com/onuroluc/vitals/releases/latest/download/vitals-darwin-arm64.tar.gz | tar xz
sudo mv vitals /usr/local/bin/

# macOS Intel
curl -sSfL https://github.com/onuroluc/vitals/releases/latest/download/vitals-darwin-amd64.tar.gz | tar xz
sudo mv vitals /usr/local/bin/

# Linux x86_64
curl -sSfL https://github.com/onuroluc/vitals/releases/latest/download/vitals-linux-amd64.tar.gz | tar xz
sudo mv vitals /usr/local/bin/

# Linux ARM64
curl -sSfL https://github.com/onuroluc/vitals/releases/latest/download/vitals-linux-arm64.tar.gz | tar xz
sudo mv vitals /usr/local/bin/
```

## Usage

```bash
# Check current directory
vitals

# Check a specific project
vitals /path/to/project

# CI mode (no colors, exit 1 on failures)
vitals --ci

# Skip specific check categories
vitals --skip services,ports

# Generate a .vitals.toml template
vitals --init
```

## Configuration

vitals works out of the box with zero configuration. For team projects, create a `.vitals.toml` to define exact requirements:

```bash
vitals --init
```

```toml
# .vitals.toml — commit this file

[require]
node = ">=18"
python = ">=3.11"

[ports]
check = [3000, 5432, 6379]

[services]
docker = true

[services.redis]
host = "localhost"
port = 6379

[services.postgres]
host = "localhost"
port = 5432

[env]
required = ["DATABASE_URL", "REDIS_URL", "API_KEY"]
example = ".env.example"

[[commands]]
name = "db-migrations"
run = "npx prisma migrate status"
```

## Auto-Detection

vitals reads your project files to figure out what checks to run:

| File | What vitals checks |
|------|-------------------|
| `package.json` | Node.js installed, node_modules, package manager |
| `.nvmrc` / `.node-version` | Node.js version matches |
| `pyproject.toml` / `requirements.txt` | Python installed, virtualenv |
| `.python-version` | Python version matches |
| `Cargo.toml` | Rust installed |
| `rust-toolchain.toml` | Rust version matches |
| `go.mod` | Go installed, version matches |
| `Gemfile` / `.ruby-version` | Ruby installed, version matches |
| `pom.xml` / `build.gradle` | Java installed |
| `docker-compose.yml` | Docker running, services reachable |
| `.env.example` | `.env` exists, all keys present |

## Check Categories

| Category | What it checks |
|----------|---------------|
| **Runtime** | Installed runtimes match version requirements |
| **Dependencies** | node_modules, virtualenvs, lockfiles |
| **Services** | Docker running, Redis/Postgres/MySQL/MongoDB reachable |
| **Ports** | Required ports are available (shows blocking process) |
| **Environment** | .env file exists and has all required keys |
| **Commands** | Custom shell commands exit 0 |

## Project Structure

```
vitals/
├── Cargo.toml
└── src/
    ├── main.rs           # CLI entry point (clap)
    ├── lib.rs            # Module declarations
    ├── platform.rs       # OS detection, install/service hints
    ├── version.rs        # Version parsing, semver comparison
    ├── detect.rs         # Project auto-detection engine
    ├── config.rs         # .vitals.toml parser
    ├── check.rs          # CheckResult / Status types
    ├── output.rs         # Colored terminal output
    └── checks/
        ├── mod.rs        # Check orchestrator
        ├── runtime.rs    # Node, Python, Rust, Go, Ruby, Java
        ├── deps.rs       # node_modules, virtualenv
        ├── service.rs    # Docker, Redis, Postgres, MySQL, Mongo
        ├── port.rs       # Port availability + process detection
        ├── env.rs        # .env vs .env.example diffing
        └── custom.rs     # User-defined command checks
```

## How It Works

1. **Detect** — Scan project directory for config files (`package.json`, `Cargo.toml`, `docker-compose.yml`, `.env.example`, etc.)
2. **Merge** — Overlay `.vitals.toml` config on top of auto-detected requirements
3. **Check** — Run each check category: version, deps, services, ports, env, commands
4. **Report** — Print colored results with actionable fix suggestions
5. **Exit** — Return 0 if all pass, 1 if any failures (for CI integration)

## License

MIT
