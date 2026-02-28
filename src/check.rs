/// Check result status.
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pass,
    Fail,
    Warn,
    Skip,
}

/// A single check result to be displayed.
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Category grouping (e.g., "Runtime", "Dependencies").
    pub category: String,
    /// Check name (e.g., "node", "redis", "node_modules").
    pub name: String,
    /// Pass / Fail / Warn / Skip.
    pub status: Status,
    /// What was actually found (e.g., "v20.10.0", "not found").
    pub found: String,
    /// What was expected (e.g., ">=18 (.nvmrc)").
    pub expected: String,
    /// Actionable fix suggestion.
    pub fix: Option<String>,
    /// Sub-items for detail (e.g., list of missing env vars).
    pub details: Vec<String>,
}
