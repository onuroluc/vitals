use std::fs;

/// Detected host platform, used for tailoring fix suggestions.
#[derive(Debug, Clone)]
pub enum Platform {
    MacOS,
    Debian,
    Fedora,
    Arch,
    Alpine,
    Linux,
    Unknown,
}

impl Platform {
    /// Detect the current platform at runtime.
    pub fn detect() -> Self {
        if cfg!(target_os = "macos") {
            return Platform::MacOS;
        }
        if cfg!(target_os = "linux") {
            if let Ok(content) = fs::read_to_string("/etc/os-release") {
                let c = content.to_lowercase();
                if c.contains("ubuntu") || c.contains("debian") {
                    return Platform::Debian;
                }
                if c.contains("fedora") || c.contains("rhel") || c.contains("centos") {
                    return Platform::Fedora;
                }
                if c.contains("arch") {
                    return Platform::Arch;
                }
                if c.contains("alpine") {
                    return Platform::Alpine;
                }
                return Platform::Linux;
            }
            return Platform::Linux;
        }
        Platform::Unknown
    }

    /// Return a platform-specific install command for a tool.
    pub fn install_hint(&self, tool: &str) -> String {
        let pkg = match tool {
            "node" => match self {
                Platform::MacOS => "node",
                Platform::Debian => "nodejs",
                _ => "nodejs",
            },
            "python" => match self {
                Platform::MacOS => "python@3",
                Platform::Debian => "python3",
                _ => "python3",
            },
            "go" => match self {
                Platform::MacOS => "go",
                Platform::Debian => "golang-go",
                _ => "golang",
            },
            "ruby" => "ruby",
            "java" => match self {
                Platform::MacOS => "openjdk",
                Platform::Debian => "default-jdk",
                _ => "java-latest-openjdk",
            },
            "docker" => match self {
                Platform::MacOS => return "brew install --cask docker".into(),
                _ => "docker.io",
            },
            _ => tool,
        };
        match self {
            Platform::MacOS => format!("brew install {}", pkg),
            Platform::Debian => format!("sudo apt install -y {}", pkg),
            Platform::Fedora => format!("sudo dnf install -y {}", pkg),
            Platform::Arch => format!("sudo pacman -S {}", pkg),
            Platform::Alpine => format!("sudo apk add {}", pkg),
            Platform::Linux | Platform::Unknown => format!("# install {}", pkg),
        }
    }

    /// Return a platform-specific service start command.
    pub fn service_hint(&self, service: &str) -> String {
        match self {
            Platform::MacOS => format!("brew services start {}", service),
            Platform::Debian | Platform::Fedora | Platform::Arch | Platform::Linux => {
                format!("sudo systemctl start {}", service)
            }
            Platform::Alpine => format!("sudo rc-service {} start", service),
            Platform::Unknown => format!("# start {}", service),
        }
    }
}
