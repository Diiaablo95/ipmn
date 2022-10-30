use async_trait::async_trait;

#[async_trait]
pub trait IpChangeNotifier<IpAddress> {
    type Error;

    async fn notify_ip_change(&self, new_address: IpAddress) -> Result<(), Self::Error>;
}
