//! data types for filtering and querying information from qbittorrent

use derive_builder;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Getting log information
#[derive(Debug, Builder, Default, Serialize)]
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

/// Filtering enum for use in making a `TorrentRequest`
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Clone, Deserialize, Serialize, Builder, Default)]
#[builder(setter(into, strip_option))]
pub struct AddTorrent {
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

//#[derive(Debug, Builder, Serialize, Deserialize, Clone, Default)]
//#[builder(setter(into, strip_option))]
//pub struct TorrentRequest {
//    #[builder(default)]
//    filter: Option<TorrentFilter>,
//    #[builder(default)]
//    category: Option<String>,
//    #[builder(default)]
//    tag: Option<String>,
//    #[builder(default)]
//    sort: Option<String>,
//    #[builder(default)]
//    reverse: Option<bool>,
//    #[builder(default)]
//    limit: Option<u64>,
//    #[builder(default)]
//    offset: Option<i64>,
//    #[builder(default)]
//    hashes: Vec<Hash>,
//}
