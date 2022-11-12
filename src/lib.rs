mod alert;
mod check;
mod config;

use std::net::Ipv4Addr;

use alert::{telegram::TelegramNotifier, traits::IpChangeNotifier};
use check::{
    network::Ipv4Provider,
    storage::FileSystemUpdatesStorage,
    traits::{IpAddressProvider, UpdatesStorage},
    IpFetchAttemptInfo,
};
use chrono::Utc;
use config::{traits::ConfigProvider, ArgsOrEnvConfigProvider};

pub async fn main() -> Result<(), &'static str> {
    let config = ArgsOrEnvConfigProvider {}.config().await;

    let ip_address_provider = Ipv4Provider::from(config.clone());
    let attempt_storage = FileSystemUpdatesStorage::from(config.clone());
    let telegram_notifier = TelegramNotifier::from(config.clone());

    let new_ip_address = ip_address_provider.get_current_ip().await;
    let last_attempt = <FileSystemUpdatesStorage as UpdatesStorage<Ipv4Addr>>::get_last_ip_attempt(
        &attempt_storage,
    )
    .await;

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

    if config.dry_run {
        return Ok(());
    }

    let delivery_result = telegram_notifier.notify_ip_change(new_ip_address).await;

    if delivery_result.is_ok() {
        new_attempt.last_delivery_success = true;
    }

    attempt_storage.save_new_attempt(new_attempt).await;

    Ok(())
}
