use crate::api::Api;
use crate::data::*;
use crate::error::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait TorrentsApi {
    fn hashes(&self) -> String;

    async fn stop(&self, api: &Api) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        api.post_status("/api/v2/torrents/stop", &form).await
    }

    async fn start(&self, api: &Api) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        api.post_status("/api/v2/torrents/start", &form).await
    }

    async fn delete(&self, api: &Api, delete_data: bool) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        form.insert(
            "deleteFiles",
            if delete_data { "true" } else { "false" }.into(),
        );
        api.post_status("/api/v2/torrents/delete", &form).await
    }

    async fn recheck(&self, api: &Api) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        api.post_status("/api/v2/torrents/recheck", &form).await
    }

    async fn set_category(&self, api: &Api, category: &str) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        form.insert("category", category.into());
        api.post_status("/api/v2/torrents/setCategory", &form).await
    }

    async fn add_tags(&self, api: &Api, tags: &[String]) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        form.insert("tags", tags.join(","));
        api.post_status("/api/v2/torrents/addTags", &form).await
    }

    async fn remove_tags(&self, api: &Api, tags: &[String]) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        form.insert("tags", tags.join(","));
        api.post_status("/api/v2/torrents/removeTags", &form).await
    }

    async fn bottom_priority(&self, api: &Api) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        api.post_status("/api/v2/torrents/bottomPrio", &form).await
    }

    async fn top_priority(&self, api: &Api) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hashes());
        api.post_status("/api/v2/torrents/topPrio", &form).await
    }
}

impl TorrentsApi for Torrent {
    fn hashes(&self) -> String {
        self.hash.hash.clone()
    }
}

impl TorrentsApi for [Torrent] {
    fn hashes(&self) -> String {
        let refs: Vec<&str> = self.iter().map(|h| h.hash.as_str()).collect();
        refs.join("|")
    }
}

impl TorrentsApi for Vec<Torrent> {
    fn hashes(&self) -> String {
        let refs: Vec<&str> = self.iter().map(|h| h.hash.as_str()).collect();
        refs.join("|")
    }
}

impl TorrentsApi for Hash {
    fn hashes(&self) -> String {
        self.hash.clone()
    }
}

impl TorrentsApi for [Hash] {
    fn hashes(&self) -> String {
        let refs: Vec<&str> = self.iter().map(|h| h.hash.as_str()).collect();
        refs.join("|")
    }
}

impl TorrentsApi for Vec<Hash> {
    fn hashes(&self) -> String {
        let refs: Vec<&str> = self.iter().map(|h| h.hash.as_str()).collect();
        refs.join("|")
    }
}

#[async_trait]
pub trait TorrentApi {
    fn hash(&self) -> String;

    async fn properties(&self, api: &Api) -> Result<TorrentProperties> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash());
        api.post_decode("/api/v2/torrents/properties", &form).await
    }

    async fn trackers(&self, api: &Api) -> Result<Vec<Tracker>> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash());
        api.post_decode("/api/v2/torrents/trackers", &form).await
    }

    async fn contents(&self, api: &Api) -> Result<Vec<TorrentInfo>> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash());
        api.post_decode("/api/v2/torrents/files", &form).await
    }
}

impl TorrentApi for Torrent {
    fn hash(&self) -> String {
        self.hash.hash.clone()
    }
}

impl TorrentApi for Hash {
    fn hash(&self) -> String {
        self.hash.clone()
    }
}
