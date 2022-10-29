mod alert;
mod check;
mod config;

use std::path::Path;

use check::storage::IpDetails;
use chrono::Utc;

use config::{traits::ConfigProvider, ArgsOrEnvConfigProvider};

pub async fn main() -> Result<(), &'static str> {
    let config = ArgsOrEnvConfigProvider {}.config().await;
    let check_file_path = Path::new(&config.check_file_path);

    let new_ip_address = check::network::new_ip_address(config.url_endpoint)
        .await
        .map_err(|_| "IP address could not be retrieved.")?;
    let old_info = check::storage::old_ip_address(check_file_path);
    if let Some(old_info) = old_info {
        // Skip only if IP has not changed and change notification was correctly delivered.
        if old_info.ip_address() == &new_ip_address && *old_info.last_delivery_success() {
            println!("IP address did not change since {}.", old_info.datetime());
            return Ok(());
        }
    }
    // Update and notify
    let new_details = IpDetails::new(new_ip_address, Utc::now());

    check::storage::write_new_ip_address(new_details, check_file_path);
    alert::telegram::notify_new_ip_address(new_ip_address, &config.tg_token, &config.chat_id)
        .await?;
    check::storage::mark_notification_delivery(check_file_path);

    Ok(())
}
