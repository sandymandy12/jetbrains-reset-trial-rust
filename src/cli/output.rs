use crate::core::{ProductInstall, ResetReport, TrialStatus};
use colored::Colorize;

pub fn print_table(products: &[ProductInstall]) {
    if products.is_empty() {
        println!("{}", "No JetBrains products found.".yellow());
        return;
    }

    println!("\n{}", "Installed JetBrains Products:".bold().cyan());
    println!("{}", "─".repeat(80));

    for product in products {
        let status_str = format_status(&product.trial_status);
        println!(
            "{} {} {} │ {}",
            product.product.icon(),
            product.product.name().bold(),
            format!("({})", product.version).dimmed(),
            status_str
        );
    }

    println!("{}\n", "─".repeat(80));
}

pub fn print_json(products: &[ProductInstall]) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(products)?;
    println!("{}", json);
    Ok(())
}

pub fn print_rofi(products: &[ProductInstall]) {
    for product in products {
        let status = match &product.trial_status {
            TrialStatus::Active { days_remaining } => format!("[{} days]", days_remaining),
            TrialStatus::Expired => "[EXPIRED]".to_string(),
            TrialStatus::NotStarted => "[Not Started]".to_string(),
            TrialStatus::Licensed => "[Licensed]".to_string(),
            TrialStatus::Unknown => "[Unknown]".to_string(),
        };

        println!(
            "{} {} {} → Reset",
            product.product.icon(),
            product.product.name(),
            status
        );
    }
}

pub fn print_status(products: &[ProductInstall]) {
    if products.is_empty() {
        println!("{}", "No JetBrains products found.".yellow());
        return;
    }

    println!("\n{}", "JetBrains Products Status:".bold().cyan());
    println!("{}", "═".repeat(80));

    for product in products {
        println!(
            "\n{} {}",
            product.product.icon(),
            product.product.name().bold()
        );
        println!("  Version: {}", product.version);
        println!("  Config:  {}", product.config_path.display());
        println!("  Status:  {}", format_status(&product.trial_status));
    }

    println!("\n{}\n", "═".repeat(80));
}

pub fn print_reset_results(reports: &[ResetReport]) {
    println!("\n{}", "Reset Results:".bold().cyan());
    println!("{}", "═".repeat(80));

    for report in reports {
        let status_icon = if report.success { "✓" } else { "✗" };
        let status_color = if report.success {
            status_icon.green()
        } else {
            status_icon.red()
        };

        println!(
            "\n{} {} {}",
            status_color,
            report.product.product.name().bold(),
            format!("({})", report.product.version).dimmed()
        );

        if report.backup_created {
            if let Some(backup_path) = &report.backup_path {
                println!(
                    "  {} Backup: {}",
                    "💾".dimmed(),
                    backup_path.display().to_string().dimmed()
                );
            }
        }

        if !report.files_modified.is_empty() {
            println!(
                "  {} Modified {} file(s)",
                "📝".dimmed(),
                report.files_modified.len()
            );
        }

        if !report.dirs_removed.is_empty() {
            println!(
                "  {} Removed {} directory(s)",
                "🗑️".dimmed(),
                report.dirs_removed.len()
            );
        }

        if let Some(error) = &report.error {
            println!("  ⚠️ Error: {}", error.red());
        }
    }

    println!("\n{}", "═".repeat(80));

    let success_count = reports.iter().filter(|r| r.success).count();
    let total_count = reports.len();

    if success_count == total_count {
        println!(
            "\n{} Successfully reset {} product(s)!\n",
            "✓".green().bold(),
            success_count
        );
    } else {
        println!(
            "\n{} Reset {}/{} products (with {} error(s))\n",
            "⚠️".yellow(),
            success_count,
            total_count,
            total_count - success_count
        );
    }
}

pub fn print_backups(backups: &[crate::core::backup::BackupInfo]) {
    if backups.is_empty() {
        println!("{}", "No backups found.".yellow());
        return;
    }

    println!("\n{}", "Available Backups:".bold().cyan());
    println!("{}", "─".repeat(80));

    for backup in backups {
        let created = chrono::DateTime::<chrono::Local>::from(backup.created);
        println!(
            "📦 {} │ {}",
            backup.name.bold(),
            created.format("%Y-%m-%d %H:%M:%S").to_string().dimmed()
        );
    }

    println!("{}\n", "─".repeat(80));
}

fn format_status(status: &TrialStatus) -> colored::ColoredString {
    match status {
        TrialStatus::Active { days_remaining } => {
            let color = if *days_remaining <= 7 {
                format!("Active ({} days)", days_remaining).red()
            } else if *days_remaining <= 15 {
                format!("Active ({} days)", days_remaining).yellow()
            } else {
                format!("Active ({} days)", days_remaining).green()
            };
            color.bold()
        }
        TrialStatus::Expired => "Expired".red().bold(),
        TrialStatus::NotStarted => "Not Started".blue(),
        TrialStatus::Licensed => "Licensed".green().bold(),
        TrialStatus::Unknown => "Unknown".dimmed(),
    }
}
