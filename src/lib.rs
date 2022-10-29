mod alert;
mod check;
mod config;

use check::{
    network::Ipv4Provider,
    storage::FileSystemUpdatesStorage,
    traits::{IpAddressProvider, IpFetchAttemptInfo, UpdatesStorage},
};
use chrono::Utc;
use config::{traits::ConfigProvider, ArgsOrEnvConfigProvider};

pub async fn main() -> Result<(), &'static str> {
    let config = ArgsOrEnvConfigProvider {}.config().await;

    let ip_address_provider = Ipv4Provider::from(config.clone());
    let attempt_storage = FileSystemUpdatesStorage::from(config.clone());

    let new_ip_address = ip_address_provider.get_current_ip().await;
    let last_attempt = attempt_storage.get_last_ip_attempt().await;

    if let Some(attempt) = last_attempt {
        // Skip only if IP has not changed and change notification was correctly delivered.
        if attempt.ip_address == new_ip_address && attempt.last_delivery_success {
            println!("IP address did not change since {}.", attempt.datetime);
            return Ok(());
        }
    }

    // Update and notify
    let mut new_attempt = IpFetchAttemptInfo {
        datetime: Utc::now(),
        ip_address: new_ip_address,
        last_delivery_success: false,
    };

    let delivery_result = alert::telegram::notify_new_ip_address(
        new_ip_address.into(),
        &config.tg_token,
        &config.chat_id,
    )
    .await;

    if delivery_result.is_ok() {
        new_attempt.last_delivery_success = true;
    }

    attempt_storage.save_new_attempt(new_attempt).await;

    Ok(())
}
