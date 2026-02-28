pub mod custom;
pub mod deps;
pub mod env;
pub mod port;
pub mod runtime;
pub mod service;

use crate::check::CheckResult;
use crate::config::VitalsConfig;
use crate::detect::ProjectContext;
use crate::platform::Platform;

/// Run all enabled checks and return results grouped by category.
pub fn run_all(
    ctx: &ProjectContext,
    config: &VitalsConfig,
    platform: &Platform,
    skip: &[String],
) -> Vec<CheckResult> {
    let mut results = Vec::new();

    if !skip.contains(&"runtime".to_string()) {
        results.extend(runtime::check(ctx, config, platform));
    }
    if !skip.contains(&"deps".to_string()) {
        results.extend(deps::check(ctx));
    }
    if !skip.contains(&"services".to_string()) {
        results.extend(service::check(ctx, config, platform));
    }
    if !skip.contains(&"ports".to_string()) {
        results.extend(port::check(ctx, config));
    }
    if !skip.contains(&"env".to_string()) {
        results.extend(env::check(ctx, config));
    }
    if !skip.contains(&"commands".to_string()) {
        results.extend(custom::check(ctx, config));
    }

    results
}
