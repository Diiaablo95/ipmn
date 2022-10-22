use std::net::IpAddr;
use telegram_bot::{Api, ChatId, SendMessage};

pub async fn notify_new_ip_address(
    ip: IpAddr,
    tg_token: &str,
    chat_id: &str,
) -> Result<(), &'static str> {
    let parsed_chat_id: i64 = chat_id.parse().expect("Bad chat ID provided.");

    let tg_api = Api::new(tg_token);
    let message = SendMessage::new(
        ChatId::new(parsed_chat_id),
        format!("Public IP address changed to {ip}."),
    );
    println!("Sending notification to TG chat.");
    tg_api
        .send(message)
        .await
        .map_err(|_| "Failed to delivery the TG notification.")
        .map(|_| {
            println!("TG notification delivered successfully.");
        })
}
