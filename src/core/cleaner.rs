use super::backup::BackupManager;
use super::product::{ProductInstall, ResetReport};
use super::xml_parser::remove_eval_keys;
use anyhow::Result;
use std::fs;

pub struct ResetOperation {
    pub backup_enabled: bool,
    pub dry_run: bool,
    backup_manager: BackupManager,
}

impl ResetOperation {
    pub fn new(backup_enabled: bool, dry_run: bool) -> Result<Self> {
        Ok(Self {
            backup_enabled,
            dry_run,
            backup_manager: BackupManager::new()?,
        })
    }

    pub fn reset_product(&self, product: &ProductInstall) -> Result<ResetReport> {
        let mut report = ResetReport::new(product.clone());

        if self.dry_run {
            println!("DRY RUN: Would reset {}", product.product.name());
            report.success = true;
            return Ok(report);
        }

        // Step 1: Create backup if enabled
        if self.backup_enabled {
            match self.backup_manager.create_backup(
                &product.config_path,
                &format!("{}_{}", product.product.name(), product.version),
            ) {
                Ok(backup_path) => {
                    report.backup_created = true;
                    report.backup_path = Some(backup_path);
                }
                Err(e) => {
                    report.error = Some(format!("Backup failed: {}", e));
                    return Ok(report);
                }
            }
        }

        // Step 2: Clean other.xml (remove eval keys)
        let other_xml = product.config_path.join("options").join("other.xml");
        if other_xml.exists() {
            if let Err(e) = remove_eval_keys(&other_xml) {
                report.error = Some(format!("Failed to clean XML: {}", e));
                return Ok(report);
            }
            report.files_modified.push(other_xml);
        }

        // Step 3: Remove eval directory
        let eval_dir = product.config_path.join("eval");
        if eval_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&eval_dir) {
                report.error = Some(format!("Failed to remove eval dir: {}", e));
                return Ok(report);
            }
            report.dirs_removed.push(eval_dir);
        }

        // Step 4: Clean data directory cache
        if product.data_path.exists() {
            // Only remove specific subdirs to be safe
            for subdir in ["eval", "port", "port.lock"].iter() {
                let path = product.data_path.join(subdir);
                if path.exists() {
                    if path.is_dir() {
                        let _ = fs::remove_dir_all(&path);
                    } else {
                        let _ = fs::remove_file(&path);
                    }
                    report.dirs_removed.push(path);
                }
            }
        }

        // Step 5: Clean Java preferences
        self.clean_java_prefs(product, &mut report)?;

        report.success = true;
        Ok(report)
    }

    fn clean_java_prefs(&self, _product: &ProductInstall, _report: &mut ResetReport) -> Result<()> {
        // Check for Java preferences
        if let Some(home) = dirs::home_dir() {
            let prefs_dir = home.join(".java").join(".userPrefs");

            if prefs_dir.exists() {
                // Search for jetbrains-related prefs and remove them
                // This is optional as not all systems use Java prefs
                let _ = self.search_and_clean_prefs(&prefs_dir);
            }
        }

        Ok(())
    }

    fn search_and_clean_prefs(&self, prefs_dir: &std::path::Path) -> Result<()> {
        for entry in fs::read_dir(prefs_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Remove jetbrains-related directories
                if dir_name.contains("jetbrains") || dir_name.contains("intellij") {
                    let _ = fs::remove_dir_all(&path);
                }

                // Recursively search
                let _ = search_and_clean_prefs_recursive(&path);
            }
        }

        Ok(())
    }

    pub fn reset_all(&self, products: &[ProductInstall]) -> Result<Vec<ResetReport>> {
        let mut reports = Vec::new();

        for product in products {
            let report = self.reset_product(product)?;
            reports.push(report);
        }

        Ok(reports)
    }
}

// Helper function for recursion to avoid clippy warning
fn search_and_clean_prefs_recursive(prefs_dir: &std::path::Path) -> Result<()> {
    for entry in fs::read_dir(prefs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if dir_name.contains("jetbrains") || dir_name.contains("intellij") {
                let _ = fs::remove_dir_all(&path);
            }

            let _ = search_and_clean_prefs_recursive(&path);
        }
    }

    Ok(())
}

impl Default for ResetOperation {
    fn default() -> Self {
        Self::new(true, false).expect("Failed to create default ResetOperation")
    }
}
