use anyhow::Result;
use notify_rust::Notification;

#[allow(dead_code)]
pub fn send_notification(title: &str, body: &str) -> Result<()> {
    Notification::new()
        .summary(title)
        .body(body)
        .icon("jetbrains-toolbox")
        .timeout(5000)
        .show()?;

    Ok(())
}

#[allow(dead_code)]
pub fn notify_reset_success(product_name: &str, count: usize) {
    let body = if count == 1 {
        format!("{} trial has been reset successfully", product_name)
    } else {
        format!("{} products have been reset successfully", count)
    };

    let _ = send_notification("Trial Reset Complete", &body);
}

#[allow(dead_code)]
pub fn notify_reset_error(error: &str) {
    let _ = send_notification("Trial Reset Failed", error);
}
