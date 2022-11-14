use async_trait::async_trait;
use std::fmt::Debug;
use telegram_bot::{Api, ChatId, SendMessage};

use crate::config::Config;

use super::traits::IpChangeNotifier;

pub struct TelegramNotifier {
    chat_id: i64,
    api: Api,
}

impl From<Config> for TelegramNotifier {
    fn from(config: Config) -> Self {
        Self {
            chat_id: config.chat_id.parse().expect("Bad chat ID provided."),
            api: Api::new(config.tg_token),
        }
    }
}

#[async_trait]
impl<IpAddress> IpChangeNotifier<IpAddress> for TelegramNotifier
where
    for<'async_trait> IpAddress: Send + Sync + 'async_trait + Debug,
{
    type Error = &'static str;

    async fn notify_ip_change(&self, new_address: IpAddress) -> Result<(), Self::Error> {
        let message = SendMessage::new(
            ChatId::new(self.chat_id),
            format!("Public IP address changed to {:?}.", new_address),
        );
        log::info!("Sending notification to TG chat.");

        self.api
            .send(message)
            .await
            .map_err(|_| "Failed to delivery the Telegram notification.")
            .map(|_| log::info!("Notification sent."))
    }
}
