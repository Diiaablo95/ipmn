use async_trait::async_trait;

use super::{Config, PartialConfig};

#[async_trait]
pub trait PartialConfigProvider {
    async fn partial_config(&self) -> PartialConfig;
}

#[async_trait]
pub trait ConfigProvider {
    async fn config(&self) -> Config;
}
