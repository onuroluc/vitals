use colored::*;

use crate::check::{CheckResult, Status};

/// Print all check results grouped by category. Returns (passed, failed).
pub fn print_results(results: &[CheckResult], ci: bool) -> (usize, usize) {
    if ci {
        colored::control::set_override(false);
    }

    // Banner
    println!();
    println!("  {} {}", "vitals".bold(), "— project health check".dimmed());
    println!("  {}", "══════════════════════════════".dimmed());

    if results.is_empty() {
        println!();
        println!(
            "  {} No project files detected.",
            "○".dimmed()
        );
        println!(
            "  Run {} to create a config, or run vitals in a project directory.",
            "vitals --init".bold()
        );
        println!();
        return (0, 0);
    }

    let mut current_category = String::new();
    let mut passed = 0usize;
    let mut failed = 0usize;

    for result in results {
        // Category header
        if result.category != current_category {
            println!();
            println!("  {}", result.category.bold().underline());
            current_category.clone_from(&result.category);
        }

        let icon = match result.status {
            Status::Pass => "✓".green().bold(),
            Status::Fail => "✗".red().bold(),
            Status::Warn => "⚠".yellow().bold(),
            Status::Skip => "○".dimmed(),
        };

        match result.status {
            Status::Pass => passed += 1,
            Status::Fail => failed += 1,
            _ => {}
        }

        // Found value — colored by status
        let found_colored = match result.status {
            Status::Pass => result.found.green().to_string(),
            Status::Fail => result.found.red().to_string(),
            Status::Warn => result.found.yellow().to_string(),
            Status::Skip => result.found.dimmed().to_string(),
        };

        // Expected — show for passing version checks (informational)
        let expected_str = if result.status == Status::Pass
            && !result.expected.is_empty()
            && result.expected != "installed"
        {
            format!("  {}", result.expected.dimmed())
        } else {
            String::new()
        };

        // Fix suggestion — show for failures
        let fix_str = match &result.fix {
            Some(fix) => format!("  {} {}", "→".dimmed(), fix.yellow()),
            None => String::new(),
        };

        println!(
            "  {} {:<14} {:<22}{}{}",
            icon, result.name, found_colored, expected_str, fix_str
        );

        // Detail lines (e.g., missing env vars)
        for detail in &result.details {
            println!("    {}", detail.dimmed());
        }
    }

    // Summary
    let total = passed + failed;
    println!();
    println!("  {}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".dimmed());
    if failed == 0 {
        println!(
            "  {} {}",
            "✓".green().bold(),
            format!("All {} checks passed", total).green().bold()
        );
    } else {
        println!(
            "  {}/{} checks passed — {} {}",
            passed.to_string().green().bold(),
            total,
            failed.to_string().red().bold(),
            if failed == 1 {
                "issue found"
            } else {
                "issues found"
            }
            .red()
        );
    }
    println!();

    (passed, failed)
}
