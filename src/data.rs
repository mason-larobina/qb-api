//! Structs returned by api queries

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

/// Overall metadata about this qbit client
#[derive(Debug, Deserialize, Getters)]
pub struct MainData {
    rid: u64,
    full_update: bool,
    torrents: Torrent,
    torrents_removed: Vec<String>,
    categories: Category,
    categories_removed: Vec<String>,
    tags: Vec<String>,
    tags_removed: Vec<String>,
    queueing: bool,
    server_state: ServerState,
}

#[derive(Debug, Deserialize, Getters, Clone)]
pub struct Torrent {
    added_on: u32,
    amount_left: u64,
    auto_tmm: bool,
    category: String,
    completed: i64,
    completion_on: i32,
    dl_limit: i64,
    dlspeed: i64,
    downloaded: i64,
    downloaded_session: i64,
    eta: i64,
    f_l_piece_prio: Option<bool>,
    force_start: bool,
    pub(crate) hash: Hash,
    last_activity: i64,
    magnet_uri: String,
    max_ratio: f64,
    max_seeding_time: i64,
    name: String,
    num_complete: i64,
    num_incomplete: i64,
    num_leechs: i64,
    num_seeds: i64,
    priority: i64,
    progress: f64,
    ratio: f64,
    ratio_limit: f64,
    save_path: String,
    seeding_time_limit: i64,
    seen_complete: i64,
    seq_dl: bool,
    size: i64,
    state: State,
    super_seeding: bool,
    tags: String,
    time_active: i64,
    total_size: i64,
    tracker: String,
    up_limit: i64,
    uploaded: i64,
    uploaded_session: i64,
    upspeed: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Getters)]
pub struct Tracker {
    url: String,
    #[getter(skip)]
    status: i32,
    // TODO: fix this since some people do things non standard with "/" here
    // tier: u32,
    num_peers: i32,
    // TODO: documentation says these will be returned but the fields do not appear
    // num_seeds: i32,
    // num_leeches: i32,
    // num_downloaded: i64,
    msg: String,
}

impl Tracker {
    pub fn status(&self) -> TrackerStatus {
        match self.status {
            0 => TrackerStatus::TrackerDisabled,
            1 => TrackerStatus::NotContacted,
            2 => TrackerStatus::Working,
            3 => TrackerStatus::Updating,
            4 => TrackerStatus::NotWorking,
            _ => TrackerStatus::UnknownResponse,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TrackerStatus {
    TrackerDisabled,
    NotContacted,
    Working,
    Updating,
    NotWorking,
    UnknownResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize, Getters)]
pub struct TorrentProperties {
    save_path: String,
    creation_date: u32,
    piece_size: i64,
    comment: String,
    total_wasted: i64,
    total_uploaded: i64,
    total_uploaded_session: i64,
    total_downloaded: i64,
    total_downloaded_session: i64,
    up_limit: i64,
    dl_limit: i64,
    time_elapsed: i64,
    seeding_time: i64,
    nb_connections: i64,
    nb_connections_limit: i64,
    share_ratio: f64,
    addition_date: i64,
    completion_date: i64,
    created_by: String,
    dl_speed_avg: i64,
    dl_speed: i64,
    eta: i64,
    last_seen: i64,
    peers: i64,
    peers_total: i64,
    pieces_have: u64,
    pieces_num: i64,
    reannounce: i64,
    seeds: i64,
    seeds_total: i64,
    total_size: u64,
    up_speed_avg: i64,
    up_speed: i64,
}

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum State {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "missingFiles")]
    MissingFiles,
    #[serde(rename = "uploading")]
    Uploading,
    #[serde(rename = "stoppedUP")]
    StoppedUP,
    #[serde(rename = "queuedUP")]
    QueuedUP,
    #[serde(rename = "stalledUP")]
    StalledUP,
    #[serde(rename = "checkingUP")]
    CheckingUP,
    #[serde(rename = "forcedUP")]
    ForcedUP,
    #[serde(rename = "allocating")]
    Allocating,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "metaDL")]
    MetaDL,
    #[serde(rename = "stoppedDL")]
    StoppedDL,
    #[serde(rename = "queuedDL")]
    QueuedDL,
    #[serde(rename = "stalledDL")]
    StalledDL,
    #[serde(rename = "checkingDL")]
    CheckingDL,
    #[serde(rename = "forcedDL")]
    ForceDL,
    #[serde(rename = "checkingResumeData")]
    CheckingResumeData,
    #[serde(rename = "moving")]
    Moving,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Deserialize, Getters)]
pub struct TransferInfo {
    dl_info_speed: u64,
    dl_info_data: u64,
    up_info_speed: u64,
    up_info_data: u64,
    dl_rate_limit: u64,
    up_rate_limit: u64,
    dht_nodes: u64,
    connection_status: ConnectionStatus,
}

#[derive(Debug, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    Disconnected,
}

#[derive(Debug, Deserialize, Getters)]
pub struct GlobalTransferInfo {
    /// Global download rate (bytes/s)
    dl_info_speed: i64,
    /// Data downloaded this session (bytes)
    dl_info_data: i64,
    /// Global upload rate (bytes/s)
    up_info_speed: i64,
    /// Data uploaded this session (bytes)
    up_info_data: i64,
    /// Download rate limit (bytes/s)
    dl_rate_limit: i64,
    /// Upload rate limit (bytes/s)
    up_rate_limit: i64,
    /// DHT nodes connected to
    dht_nodes: i64,
    connection_status: ConnectionStatus,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlternateLimits {
    /// Alternative limits are in effect
    Enabled,
    /// Run at full speed
    Disabled,
}

#[derive(Debug, Deserialize, Serialize, Getters)]
pub struct TorrentInfo {
    hash: Hash,
    name: String,
    size: i64,
    progress: f64,
    priority: i16,
    is_seed: Option<bool>,
    piece_range: Vec<i64>,
    availability: f64,
}

#[derive(Debug, Deserialize, Default, Getters)]
pub struct Category {
    name: String,
    #[serde(rename = "savePath")]
    save_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerState {}

#[derive(Debug, Deserialize)]
pub struct Peer {}

#[derive(Debug, Deserialize, Getters)]
pub struct BuildInfo {
    qt: String,
    libtorrent: String,
    boost: String,
    openssl: String,
    bitness: i64,
}

#[derive(Deserialize, Debug)]
pub struct Preferences {}

#[derive(Deserialize, Debug, Getters)]
pub struct Log {
    id: u64,
    message: String,
    timestamp: u64,
    #[serde(rename = "type")]
    level: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, Hash)]
#[serde(transparent)]
pub struct Hash {
    pub(crate) hash: String,
}

impl From<String> for Hash {
    fn from(f: String) -> Self {
        Hash { hash: f }
    }
}

impl std::ops::Deref for Hash {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.hash
    }
}
