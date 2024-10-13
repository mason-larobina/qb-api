//! data types for filtering and querying information from qbittorrent

use crate::api::Api;
use crate::data::{Hash, Torrent};
use crate::error::{self, Error};
use derive_builder;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Getting log information
#[derive(Debug, Builder, Default)]
pub struct LogRequest {
    #[builder(default)]
    normal: bool,
    #[builder(default)]
    info: bool,
    #[builder(default)]
    warning: bool,
    #[builder(default)]
    critical: bool,
    #[builder(default)]
    last_known_id: u64,
}

impl LogRequest {
    fn bool_to_string(b: bool) -> String {
        if b {
            String::from("true")
        } else {
            String::from("false")
        }
    }

    pub(crate) fn to_form_data(&self) -> HashMap<&'static str, String> {
        let mut data = HashMap::new();
        data.insert("normal", Self::bool_to_string(self.normal));
        data.insert("info", Self::bool_to_string(self.info));
        data.insert("warning", Self::bool_to_string(self.warning));
        data.insert("critical", Self::bool_to_string(self.critical));
        data.insert("last_known_id", self.last_known_id.to_string());
        data
    }
}

/// filter optional 	Filter torrent list. Allowed filters: all, downloading, completed, paused, active, inactive, 'resumed'
/// category optional 	Get torrents with the given category (empty string means "without category"; no "category" parameter means "any category")
/// sort optional 	Sort torrents by given key. All the possible keys are listed here below
/// reverse optional 	Enable reverse sorting. Possible values are true and false (default)
/// limit optional 	Limit the number of torrents returned
/// offset optional 	Set offset (if less than 0, offset from end)
/// hashes optional 	Filter by hashes. Can contain multiple hashes separated by |
#[derive(Debug, Builder, Serialize, Deserialize, Clone, Default)]
#[builder(setter(into, strip_option))]
pub struct TorrentRequest {
    #[builder(default)]
    filter: Option<TorrentFilter>,
    #[builder(default)]
    category: Option<String>,
    #[builder(default)]
    tag: Option<String>,
    #[builder(default)]
    sort: Option<String>,
    #[builder(default)]
    reverse: Option<bool>,
    #[builder(default)]
    limit: Option<u64>,
    #[builder(default)]
    offset: Option<i64>,
    #[builder(default)]
    hashes: Vec<Hash>,
}

impl TorrentRequest {
    pub async fn send(self, api: &Api) -> Result<Vec<Torrent>, Error> {
        let mut form = HashMap::new();
        if let Some(filter) = &self.filter {
            form.insert("filter", serde_json::to_string(filter)?);
        }
        if let Some(category) = &self.category {
            form.insert("category", category.clone());
        }
        if let Some(tag) = &self.tag {
            form.insert("tag", tag.clone());
        }
        if let Some(sort) = &self.sort {
            form.insert("sort", sort.clone());
        }
        if let Some(reverse) = &self.reverse {
            form.insert("reverse", serde_json::to_string(reverse)?);
        }
        if let Some(limit) = &self.limit {
            form.insert("limit", limit.to_string());
        }
        if let Some(offset) = &self.offset {
            form.insert("offset", offset.to_string());
        }
        if !self.hashes.is_empty() {
            let refs: Vec<&str> = self.hashes.iter().map(|h| h.hash.as_str()).collect();
            form.insert("hashes", refs.join("|"));
        }
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/info"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let torrents: Vec<Torrent> = serde_json::from_slice(&data)?;
        Ok(torrents)
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// Filtering enum for use in making a `TorrentRequest`
pub enum TorrentFilter {
    #[serde(rename = "all")]
    #[default]
    All,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "seeding")]
    Seeding,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "resumed")]
    Resumed,
    #[serde(rename = "stalled")]
    Stalled,
    #[serde(rename = "stalled_uploading")]
    StalledUploading,
    #[serde(rename = "stalled_downloading")]
    StalledDownloading,
    #[serde(rename = "errored")]
    Errored,
}

/// Metadata for downloading magnet links and torrent files
///
/// NOTE: You must include either a `urls` field or `torrents` field
///
/// urls 	string 	URLs separated with newlines
/// torrents 	raw 	Raw data of torrent file. torrents can be presented multiple times.
/// savepath optional 	string 	Download folder
/// cookie optional 	string 	Cookie sent to download the .torrent file
/// category optional 	string 	Category for the torrent
/// skip_checking optional 	string 	Skip hash checking. Possible values are true, false (default)
/// paused optional 	string 	Add torrents in the paused state. Possible values are true, false (default)
/// root_folder optional 	string 	Create the root folder. Possible values are true, false, unset (default)
/// rename optional 	string 	Rename torrent
/// upLimit optional 	integer 	Set torrent upload speed limit. Unit in bytes/second
/// dlLimit optional 	integer 	Set torrent download speed limit. Unit in bytes/second
/// autoTMM optional 	bool 	Whether Automatic Torrent Management should be used
/// sequentialDownload optional 	string 	Enable sequential download. Possible values are true, false (default)
/// firstLastPiecePrio optional 	string 	Prioritize download first last piece. Possible values are true, false (default)
#[derive(Debug, Clone, Deserialize, Serialize, Builder, Default)]
#[builder(setter(into, strip_option))]
pub struct TorrentDownload {
    #[builder(default)]
    urls: Option<String>,
    #[builder(default)]
    torrents: Option<Vec<u8>>,
    #[builder(default)]
    savepath: Option<String>,
    #[builder(default)]
    cookie: Option<String>,
    #[builder(default)]
    category: Option<String>,
    #[builder(default)]
    skip_checking: Option<String>,
    #[builder(default)]
    paused: Option<String>,
    #[builder(default)]
    root_folder: Option<String>,
    #[builder(default)]
    rename: Option<String>,
    #[builder(default)]
    #[serde(rename = "upLimit")]
    upload_limit: Option<i64>,
    #[builder(default)]
    #[serde(rename = "dlLimit")]
    download_limit: Option<i64>,
    #[builder(default)]
    #[serde(rename = "autoTMM")]
    automatic_management: Option<bool>,
    #[builder(default)]
    #[serde(rename = "sequentialDownload")]
    sequential_download: Option<String>,
    #[builder(default)]
    #[serde(rename = "firstLastPiecePrio")]
    first_last_piece_prio: Option<String>,
}

impl TorrentDownload {
    pub async fn download(&self, api: &Api) -> Result<(), error::Error> {
        api.add_new_torrent(self).await
    }
}
