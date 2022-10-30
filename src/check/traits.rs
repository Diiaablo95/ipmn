use async_trait::async_trait;

use super::IpFetchAttemptInfo;

#[async_trait]
pub(crate) trait IpAddressProvider<IpAddress> {
    async fn get_current_ip(&self) -> IpAddress;
}

#[async_trait]
pub(crate) trait UpdatesStorage<IpAddress> {
    async fn get_last_ip_attempt(&self) -> Option<IpFetchAttemptInfo<IpAddress>>;
    async fn save_new_attempt(&self, new_attempt: IpFetchAttemptInfo<IpAddress>);
}
