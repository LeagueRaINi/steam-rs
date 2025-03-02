//! # Implements the `GetAppList` endpoint

use serde::{Deserialize, Serialize};

use crate::{
    errors::{ErrorHandle, StoreServiceError},
    macros::{do_http, optional_argument},
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetAppList";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
struct Wrapper {
    response: AppList,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppList {
    pub apps: Vec<App>,
    pub have_more_results: Option<bool>,
    pub last_appid: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct App {
    pub appid: u32,
    pub name: String,
    pub last_modified: u32,
    pub price_change_number: u32,
}

// TODO: Might be better to implement a builder pattern for this one.
// ```rust
// let app_list = steam.get_store_app_list()
//     .if_modified_since(0)
//     .have_description_language("english")
//     .include_games(true)
//     .include_dlc(true)
//     .include_software(true)
//     .include_videos(true)
//     .include_hardware(true)
//     .last_appid(0)
//     .max_results(10000)
//     .execute()
//     .await?;
// ```
impl Steam {
    /// Gets a list of apps available on the Steam Store.
    /// 
    /// # Arguments
    /// 
    /// * `if_modified_since` - Return only items that have been modified since this date.
    /// * `have_description_language` - Return only items that have a description in this language.
    /// * `include_games` - Include games (defaults to enabled).
    /// * `include_dlc` - Include DLC.
    /// * `include_software` - Include software items.
    /// * `include_videos` - Include videos and series.
    /// * `include_hardware` - Include hardware.
    /// * `last_appid` - For continuations, this is the last appid returned from the previous call.
    /// * `max_results` - Number of results to return at a time. Default 10k, max 50k.
    pub async fn get_store_app_list(
        &self,
        if_modified_since: Option<u32>,
        have_description_language: Option<String>,
        include_games: Option<bool>,
        include_dlc: Option<bool>,
        include_software: Option<bool>,
        include_videos: Option<bool>,
        include_hardware: Option<bool>,
        last_appid: Option<u32>,
        max_results: Option<u32>,
    ) -> Result<AppList, StoreServiceError> {
        let key = &self.api_key;
        let query = format!(
            "?key={}{}{}{}{}{}{}{}{}{}",
            key,
            optional_argument!(if_modified_since, "if_modified_since"),
            optional_argument!(have_description_language, "have_description_language"),
            optional_argument!(include_games, "include_games"),
            optional_argument!(include_dlc, "include_dlc"),
            optional_argument!(include_software, "include_software"),
            optional_argument!(include_videos, "include_videos"),
            optional_argument!(include_hardware, "include_hardware"),
            optional_argument!(last_appid, "last_appid"),
            optional_argument!(max_results, "max_results")
        );

        let url = format!("{}/{}/{}/v{}/{}", BASE, INTERFACE, ENDPOINT, VERSION, query);
        let wrapper = do_http!(
            url,
            Wrapper,
            ErrorHandle,
            StoreServiceError::GetAppList
        );
        Ok(wrapper.response)
    }
}