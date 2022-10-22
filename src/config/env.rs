const TG_TOKEN_KEY: &str = "TG_TOKEN";
const CHAT_ID_KEY: &str = "CHAT_ID";
const CHECK_FILE_PATH_KEY: &str = "CHECK_FILE_PATH";
const URL_ENDPOINT_KEY: &str = "URL_ENDPOINT";

#[derive(Debug)]
pub(super) struct Config {
    pub(super) tg_token: Option<String>,
    pub(super) chat_id: Option<String>,
    pub(super) check_file_path: Option<String>,
    pub(super) url_endpoint: Option<String>,
}

impl Config {
    pub(super) fn parse() -> Self {
        let tg_token = std::env::var(TG_TOKEN_KEY).ok();
        let chat_id = std::env::var(CHAT_ID_KEY).ok();
        let check_file_path = std::env::var(CHECK_FILE_PATH_KEY).ok();
        let url_endpoint = std::env::var(URL_ENDPOINT_KEY).ok();
        Self {
            tg_token,
            chat_id,
            check_file_path,
            url_endpoint,
        }
    }
}
