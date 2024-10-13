use super::error;
use crate::data::{AlternateLimits, BuildInfo, Categories, GlobalTransferInfo, Hash, Log, Torrent};
use crate::queries::{LogRequest, TorrentDownload};
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
/// Main handle and access point to working with qbittorrent.
///
/// Full documentation on provided methods is available here
/// [here](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1))
pub struct Api {
    pub(crate) cookie: String,
    pub(crate) url: Url,
    pub(crate) client: reqwest::Client,
}

impl Api {
    pub async fn new<U: Into<Url>>(
        url: U,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self, error::Error> {
        let client = reqwest::Client::new();

        let mut url: Url = url.into();
        url.set_fragment(None);
        url.set_query(None);
        url.set_path("");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Referer", url.as_str().parse()?);

        let mut form = HashMap::new();
        form.insert("username", username);
        form.insert("password", password);

        let mut login_url = url.clone();
        login_url.set_path("/api/v2/auth/login");

        let request = dbg!(client.post(login_url).headers(headers).form(&form));
        let response = dbg!(request.send().await?);

        let headers = match response.headers().get("set-cookie") {
            Some(header) => header,
            None => return Err(error::Error::MissingHeaders),
        };

        let cookie_str = headers.to_str()?;
        let cookie_header = match cookie_str.find(";") {
            Some(index) => index,
            None => return Err(error::Error::MissingCookie),
        };

        // parse off the "SID=" portion of the cookie
        let cookie = match cookie_str.get(0..cookie_header) {
            Some(cookie) => cookie,
            None => return Err(error::Error::SliceError),
        };

        Ok(Self {
            cookie: cookie.to_string(),
            url,
            client,
        })
    }

    pub(crate) fn endpoint(&self, path: &str) -> Url {
        let mut url = self.url.clone();
        url.set_path(path);
        url
    }

    pub async fn application_version(&self) -> Result<String, error::Error> {
        let request = dbg!(self.client.get(self.endpoint("/api/v2/app/version")));
        let response = dbg!(request.send().await?);
        let text = dbg!(response.text().await?);
        Ok(text)
    }

    pub async fn api_version(&self) -> Result<String, error::Error> {
        let request = dbg!(self.client.get(self.endpoint("/api/v2/app/webapiVersion")));
        let response = dbg!(request.send().await?);
        let text = dbg!(response.text().await?);
        Ok(text)
    }

    pub async fn build_info(&self) -> Result<BuildInfo, error::Error> {
        let request = dbg!(self.client.get(self.endpoint("/api/v2/app/buildInfo")));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let info = serde_json::from_slice(&data)?;
        Ok(info)
    }

    pub async fn shutdown(&self) -> Result<(), error::Error> {
        let request = dbg!(self.client.get(self.endpoint("/api/v2/app/shutdown")));
        let _response = dbg!(request.send().await?);
        Ok(())
    }

    pub async fn default_save_path(&self) -> Result<String, error::Error> {
        let request = dbg!(self
            .client
            .get(self.endpoint("/api/v2/app/defaultSavePath")));
        let response = dbg!(request.send().await?);
        let text = dbg!(response.text().await?);
        Ok(text)
    }

    // ######
    // ###### Logging
    // ######

    pub async fn get_log(&self, log_request: &LogRequest) -> Result<Vec<Log>, error::Error> {
        let form = log_request.to_form_data();
        let request = dbg!(self
            .client
            .post(self.endpoint("/api/v2/log/main"))
            .form(&form));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let log: Vec<Log> = serde_json::from_slice(&data)?;
        Ok(log)
    }

    // #####
    // ##### Sync
    // #####

    // get_torrent_peers is a trait

    // #####
    // ##### Transfer Info
    // #####

    // /api/v2/transfer/methodName

    pub async fn get_global_transfer_info(&self) -> Result<GlobalTransferInfo, error::Error> {
        let request = dbg!(self
            .client
            .get(self.endpoint("/api/v2/transfer/info"))
            .headers(self.headers()?));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let info = serde_json::from_slice(&data)?;
        Ok(info)
    }

    pub async fn get_alternate_speed_limits_state(&self) -> Result<AlternateLimits, error::Error> {
        let request = dbg!(self
            .client
            .get(self.endpoint("/api/v2/transfer/speedLimitsMode"))
            .headers(self.headers()?));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        if data.as_ref() == b"1" {
            Ok(AlternateLimits::Enabled)
        } else if data.as_ref() == b"0" {
            Ok(AlternateLimits::Disabled)
        } else {
            Err(error::Error::BadResponse)
        }
    }

    pub async fn toggle_alternative_speed_limits(&self) -> Result<(), error::Error> {
        let request = dbg!(self
            .client
            .get(self.endpoint("/api/v2/transfer/toggleSpeedLimitsMode")));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(error::Error::from(e)),
        }
    }

    // ban_peers is a trait

    // TODO: extra filtering parameters here
    pub async fn get_torrent_list(&self) -> Result<Vec<Torrent>, error::Error> {
        let request = dbg!(self
            .client
            .get(self.endpoint("/api/v2/torrents/info"))
            .headers(self.headers()?));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let all_torrents: Vec<Torrent> = serde_json::from_slice(&data)?;
        Ok(all_torrents)
    }

    pub async fn add_new_torrent(&self, data: &TorrentDownload) -> Result<(), error::Error> {
        let request = dbg!(self
            .client
            .post(self.endpoint("/api/v2/torrents/add"))
            .form(data)
            .headers(self.headers()?));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(error::Error::from(e)),
        }
    }

    /// Make the authentication headers for each request
    pub(crate) fn headers(&self) -> Result<reqwest::header::HeaderMap, error::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("cookie", self.cookie.parse()?);
        headers.insert("Referer", self.url.as_str().parse()?);
        Ok(headers)
    }

    /// list all categories that currently exist
    pub async fn get_all_categories(&self) -> Result<HashMap<String, Categories>, error::Error> {
        let request = dbg!(self
            .client
            .get(self.endpoint("/api/v2/torrents/categories"))
            .headers(self.headers()?));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let categories = serde_json::from_slice(&data)?;
        Ok(categories)
    }

    pub async fn add_category(&self, name: &str, path: &str) -> Result<(), error::Error> {
        let mut form = HashMap::new();
        form.insert("category", name);
        form.insert("savePath", path);
        let request = dbg!(self
            .client
            .post(self.endpoint("/api/v2/torrents/createCategory"))
            .headers(self.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        let _data = dbg!(response.bytes().await?);
        Ok(())
    }

    pub(crate) fn hashes_form_data(hashes: &[Hash]) -> HashMap<&str, String> {
        let hash_strs: Vec<&str> = hashes.iter().map(|h| h.hash.as_str()).collect();
        let hashes = hash_strs.join("|");
        let mut data = HashMap::new();
        data.insert("hashes", hashes);
        data
    }

    pub async fn bottom_prio(&self, hashes: &[Hash]) -> Result<(), error::Error> {
        let request = dbg!(self
            .client
            .post(self.endpoint("/api/v2/torrents/bottomPrio"))
            .headers(self.headers()?)
            .form(&Self::hashes_form_data(hashes)));
        let response = dbg!(request.send().await?);
        let _data = dbg!(response.bytes().await?);
        Ok(())
    }

    pub async fn top_prio(&self, hashes: &[Hash]) -> Result<(), error::Error> {
        let request = dbg!(self
            .client
            .post(self.endpoint("/api/v2/torrents/topPrio"))
            .headers(self.headers()?)
            .form(&Self::hashes_form_data(hashes)));
        let response = dbg!(request.send().await?);
        let _data = dbg!(response.bytes().await?);
        Ok(())
    }
}
