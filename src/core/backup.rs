use anyhow::Result;
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

pub struct BackupManager {
    backup_root: PathBuf,
}

impl BackupManager {
    pub fn new() -> Result<Self> {
        let backup_root = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".jetbrains-trial-backups");

        if !backup_root.exists() {
            fs::create_dir_all(&backup_root)?;
        }

        Ok(Self { backup_root })
    }

    pub fn create_backup(&self, config_path: &Path, product_name: &str) -> Result<PathBuf> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("{}_{}", product_name, timestamp);
        let backup_path = self.backup_root.join(&backup_name);

        fs::create_dir_all(&backup_path)?;

        // Backup options directory
        let options_src = config_path.join("options");
        if options_src.exists() {
            let options_dst = backup_path.join("options");
            copy_dir_recursive(&options_src, &options_dst)?;
        }

        // Backup eval directory if exists
        let eval_src = config_path.join("eval");
        if eval_src.exists() {
            let eval_dst = backup_path.join("eval");
            copy_dir_recursive(&eval_src, &eval_dst)?;
        }

        Ok(backup_path)
    }

    pub fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        let mut backups = Vec::new();

        if !self.backup_root.exists() {
            return Ok(backups);
        }

        for entry in fs::read_dir(&self.backup_root)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let metadata = fs::metadata(&path)?;
                let modified = metadata.modified()?;

                backups.push(BackupInfo {
                    path: path.clone(),
                    name: path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    created: modified,
                });
            }
        }

        backups.sort_by(|a, b| b.created.cmp(&a.created));
        Ok(backups)
    }

    #[allow(dead_code)]
    pub fn restore_backup(&self, backup_path: &Path, target_path: &Path) -> Result<()> {
        // Restore options
        let options_src = backup_path.join("options");
        if options_src.exists() {
            let options_dst = target_path.join("options");
            if options_dst.exists() {
                fs::remove_dir_all(&options_dst)?;
            }
            copy_dir_recursive(&options_src, &options_dst)?;
        }

        Ok(())
    }

    pub fn delete_backup(&self, backup_path: &Path) -> Result<()> {
        fs::remove_dir_all(backup_path)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub path: PathBuf,
    pub name: String,
    pub created: std::time::SystemTime,
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
