#[tokio::main]
pub async fn main() -> Result<(), &'static str> {
    ipmn::main().await
}
