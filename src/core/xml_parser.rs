use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Remove evaluation keys from JetBrains other.xml config file
pub fn remove_eval_keys(xml_path: &Path) -> Result<()> {
    let content = fs::read_to_string(xml_path).context("Failed to read XML file")?;

    let cleaned = clean_xml_content(&content)?;

    fs::write(xml_path, cleaned).context("Failed to write cleaned XML")?;

    Ok(())
}

fn clean_xml_content(content: &str) -> Result<String> {
    let mut result = Vec::new();
    let mut skip_line = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip lines containing trial/eval keys
        if trimmed.contains("evlsprt")
            || trimmed.contains("trial.state")
            || trimmed.contains("trial.") && trimmed.contains("availability")
        {
            skip_line = true;
            continue;
        }

        // If previous line was eval key and this is a comma, skip it
        if skip_line && (trimmed == "," || trimmed.is_empty()) {
            skip_line = false;
            continue;
        }

        skip_line = false;
        result.push(line);
    }

    Ok(result.join("\n"))
}

#[allow(dead_code)]
pub fn check_has_eval_keys(xml_path: &Path) -> Result<bool> {
    let content = fs::read_to_string(xml_path)?;

    Ok(content.contains("evlsprt")
        || content.contains("trial.state")
        || content.contains("trial.availability"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_xml() {
        let input = r#"
{
  "keyToString": {
    "RunOnceActivity.TerminalFontSettingsService": "true",
    "evlsprt.252": "199a2010580",
    "evlsprt3.251": "7",
    "evlsprt3.252": "6",
    "trial.state.last.state": "ALERT",
    "experimental.ui.used.version": "252.26199.169"
  }
}
"#;

        let result = clean_xml_content(input).unwrap();

        assert!(!result.contains("evlsprt"));
        assert!(!result.contains("trial.state"));
        assert!(result.contains("experimental.ui.used.version"));
    }
}
