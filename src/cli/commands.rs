use crate::core::{scan_jetbrains_products, ResetOperation};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jb-reset")]
#[command(about = "JetBrains Trial Reset Tool for Linux", version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all installed JetBrains products
    List {
        /// Output as JSON
        #[arg(short, long)]
        json: bool,

        /// Output for rofi/dmenu
        #[arg(short, long)]
        rofi: bool,
    },

    /// Reset trial for products
    Reset {
        /// Product names to reset (e.g., intellij, pycharm)
        products: Vec<String>,

        /// Reset all products
        #[arg(short, long)]
        all: bool,

        /// Dry run (preview without making changes)
        #[arg(short, long)]
        dry_run: bool,

        /// Disable backup
        #[arg(long)]
        no_backup: bool,
    },

    /// Manage backups
    Backup {
        #[command(subcommand)]
        action: BackupAction,
    },

    /// Show status of all products
    Status,
}

#[derive(Subcommand)]
pub enum BackupAction {
    /// List all backups
    List,

    /// Delete a backup
    Delete { backup_name: String },
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { json, rofi } => {
            let products = scan_jetbrains_products()?;

            if json {
                super::output::print_json(&products)?;
            } else if rofi {
                super::output::print_rofi(&products);
            } else {
                super::output::print_table(&products);
            }
        }

        Commands::Reset {
            products: product_names,
            all,
            dry_run,
            no_backup,
        } => {
            let all_products = scan_jetbrains_products()?;

            if all_products.is_empty() {
                println!("No JetBrains products found.");
                return Ok(());
            }

            let to_reset = if all {
                all_products
            } else {
                filter_products(&all_products, &product_names)
            };

            if to_reset.is_empty() {
                println!("No matching products found.");
                return Ok(());
            }

            let operation = ResetOperation::new(!no_backup, dry_run)?;
            let reports = operation.reset_all(&to_reset)?;

            super::output::print_reset_results(&reports);
        }

        Commands::Backup { action } => match action {
            BackupAction::List => {
                let manager = crate::core::BackupManager::new()?;
                let backups = manager.list_backups()?;
                super::output::print_backups(&backups);
            }
            BackupAction::Delete { backup_name } => {
                let manager = crate::core::BackupManager::new()?;
                let backups = manager.list_backups()?;

                if let Some(backup) = backups.iter().find(|b| b.name == backup_name) {
                    manager.delete_backup(&backup.path)?;
                    println!("✓ Backup deleted: {}", backup_name);
                } else {
                    println!("Backup not found: {}", backup_name);
                }
            }
        },

        Commands::Status => {
            let products = scan_jetbrains_products()?;
            super::output::print_status(&products);
        }
    }

    Ok(())
}

fn filter_products(
    all_products: &[crate::core::ProductInstall],
    names: &[String],
) -> Vec<crate::core::ProductInstall> {
    all_products
        .iter()
        .filter(|p| {
            let product_name = p.product.name().to_lowercase();
            names
                .iter()
                .any(|n| product_name.contains(&n.to_lowercase()))
        })
        .cloned()
        .collect()
}
