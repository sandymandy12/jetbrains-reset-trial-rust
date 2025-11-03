use super::product::{JetBrainsProduct, ProductInstall, TrialStatus};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn scan_jetbrains_products() -> Result<Vec<ProductInstall>> {
    let config_base = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("JetBrains");

    if !config_base.exists() {
        return Ok(Vec::new());
    }

    let mut products = Vec::new();

    for entry in fs::read_dir(&config_base)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(product) = parse_product_dir(&path)? {
                products.push(product);
            }
        }
    }

    Ok(products)
}

fn parse_product_dir(path: &Path) -> Result<Option<ProductInstall>> {
    let dir_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid directory name")?;

    let product = match JetBrainsProduct::from_dir_name(dir_name) {
        Some(p) => p,
        None => return Ok(None),
    };

    let version = extract_version(dir_name);

    let data_path = dirs::data_local_dir()
        .context("Could not determine data directory")?
        .join("JetBrains")
        .join(dir_name);

    let trial_status = detect_trial_status(path)?;

    Ok(Some(ProductInstall {
        product,
        version,
        config_path: path.to_path_buf(),
        data_path,
        trial_status,
    }))
}

fn extract_version(dir_name: &str) -> String {
    dir_name
        .chars()
        .skip_while(|c| !c.is_numeric())
        .collect::<String>()
        .split('.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".")
}

fn detect_trial_status(config_path: &Path) -> Result<TrialStatus> {
    let other_xml = config_path.join("options").join("other.xml");

    if !other_xml.exists() {
        return Ok(TrialStatus::NotStarted);
    }

    let content = fs::read_to_string(&other_xml)?;

    // Check for license
    if content.contains("license") || content.contains("Licensed") {
        return Ok(TrialStatus::Licensed);
    }

    // Check for trial keys
    if content.contains("evlsprt") {
        // Extract days remaining from trial counter
        let days = extract_trial_days(&content).unwrap_or(0);

        if days == 0 {
            return Ok(TrialStatus::Expired);
        } else {
            return Ok(TrialStatus::Active {
                days_remaining: days,
            });
        }
    }

    // Check trial state
    if content.contains("trial.state") {
        if content.contains("\"EXPIRED\"") {
            return Ok(TrialStatus::Expired);
        } else if content.contains("\"ACTIVE\"") {
            let days = extract_trial_days(&content).unwrap_or(30);
            return Ok(TrialStatus::Active {
                days_remaining: days,
            });
        }
    }

    Ok(TrialStatus::Unknown)
}

fn extract_trial_days(content: &str) -> Option<u32> {
    // Try to find evlsprt3.XXX key which contains days remaining
    for line in content.lines() {
        if line.contains("evlsprt3") {
            if let Some(value) = line.split(':').nth(1) {
                let clean = value.trim().trim_matches('"').trim_matches(',');
                if let Ok(days) = clean.parse::<u32>() {
                    return Some(days);
                }
            }
        }
    }

    // Default to 30 if we can't parse
    Some(30)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        assert_eq!(extract_version("IntelliJIdea2025.2"), "2025.2");
        assert_eq!(extract_version("PyCharm2024.3"), "2024.3");
    }

    #[test]
    fn test_product_from_dir() {
        assert_eq!(
            JetBrainsProduct::from_dir_name("IntelliJIdea2025.2"),
            Some(JetBrainsProduct::IntelliJIdea)
        );
        assert_eq!(
            JetBrainsProduct::from_dir_name("PyCharm2024.3"),
            Some(JetBrainsProduct::PyCharm)
        );
    }
}
