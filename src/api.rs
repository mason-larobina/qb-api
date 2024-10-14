use crate::data::{AlternateLimits, BuildInfo, Category, GlobalTransferInfo, Log, Torrent};
use crate::error::{Error, Result};
use crate::queries::{AddTorrent, LogRequest};
use log::*;
use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::{HashMap, HashSet};
use url::Url;

#[derive(Debug)]
/// Main handle and access point to working with qbittorrent.
///
/// Full documentation on provided methods is available
/// [here](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1))
pub struct Api {
    pub(crate) cookie: String,
    pub(crate) url: Url,
    pub(crate) client: reqwest::Client,
}

impl Api {
    async fn new(url: &str, form: &HashMap<&str, &str>) -> Result<Self> {
        let client = reqwest::Client::new();

        let mut url: Url = Url::parse(url)?;
        url.set_fragment(None);
        url.set_query(None);
        url.set_path("");

        let mut headers = HeaderMap::new();
        headers.insert("Referer", url.as_str().parse()?);

        let mut login_url = url.clone();
        login_url.set_path("/api/v2/auth/login");

        let request = dbg!(client.post(login_url).headers(headers).form(&form));
        let response = dbg!(request.send().await?);

        let headers = match response.headers().get("set-cookie") {
            Some(header) => header,
            None => return Err(Error::MissingHeaders),
        };

        let cookie_str = headers.to_str()?;
        let cookie_header = match cookie_str.find(";") {
            Some(index) => index,
            None => return Err(Error::MissingCookie),
        };

        // parse off the "SID=" portion of the cookie
        let cookie = match cookie_str.get(0..cookie_header) {
            Some(cookie) => cookie,
            None => return Err(Error::SliceError),
        };

        Ok(Self {
            cookie: cookie.to_string(),
            url,
            client,
        })
    }

    pub async fn auth(url: &str, username: &str, password: &str) -> Result<Self> {
        let mut form = HashMap::new();
        form.insert("username", username);
        form.insert("password", password);
        Self::new(url, &form).await
    }

    pub async fn local(url: &str) -> Result<Self> {
        let form = HashMap::new();
        Self::new(url, &form).await
    }

    //
    // Making requests
    //

    pub(crate) fn endpoint(&self, path: &str) -> Url {
        let mut url = self.url.clone();
        url.set_path(path);
        url
    }

    pub(crate) fn headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", self.cookie.parse()?);
        headers.insert("Referer", self.url.as_str().parse()?);
        Ok(headers)
    }

    pub(crate) async fn post<F: Serialize + ?Sized>(&self, path: &str, form: &F) -> Result<()> {
        let request = self
            .client
            .post(self.endpoint(path))
            .headers(self.headers()?)
            .form(form);
        debug!("POST -> {:?} {:?}", path, request);
        let response = request.send().await?;
        debug!("POST <- {:?} {:?}", path, response);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub(crate) async fn post_and_decode<F: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<T> {
        let request = self
            .client
            .post(self.endpoint(path))
            .headers(self.headers()?)
            .form(form);
        debug!("POST -> {:?} {:?}", path, request);
        let response = request.send().await?;
        debug!("POST <- {:?} {:?}", path, response);
        let data = response.bytes().await?;
        debug!("DATA {:?}", std::str::from_utf8(&data));
        let ret = serde_json::from_slice(&data)?;
        Ok(ret)
    }

    pub(crate) async fn post_and_text<F: Serialize + ?Sized>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<String> {
        let request = self
            .client
            .post(self.endpoint(path))
            .headers(self.headers()?)
            .form(form);
        debug!("POST -> {:?} {:?}", path, request);
        let response = request.send().await?;
        debug!("POST <- {:?} {:?}", path, response);
        let text = response.text().await?;
        debug!("TEXT {:?}", text);
        Ok(text)
    }

    //
    // Application info / control
    //

    pub async fn get_app_version(&self) -> Result<String> {
        self.post_and_text("/api/v2/app/version", &()).await
    }

    pub async fn get_api_version(&self) -> Result<String> {
        self.post_and_text("/api/v2/app/webapiVersion", &()).await
    }

    pub async fn get_build_info(&self) -> Result<BuildInfo> {
        self.post_and_decode("/api/v2/app/buildInfo", &()).await
    }

    pub async fn get_default_save_path(&self) -> Result<String> {
        self.post_and_text("/api/v2/app/defaultSavePath", &()).await
    }

    pub async fn get_main_logs(&self, logs: &LogRequest) -> Result<Vec<Log>> {
        self.post_and_decode("/api/v2/log/main", &logs).await
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.post("/api/v2/app/shutdown", &()).await
    }

    //
    // Speed info / limits / control
    //

    pub async fn get_global_transfer_info(&self) -> Result<GlobalTransferInfo> {
        self.post_and_decode("/api/v2/transfer/info", &()).await
    }

    pub async fn get_alt_speed_limits_state(&self) -> Result<AlternateLimits> {
        let text = self
            .post_and_text("/api/v2/transfer/speedLimitsMode", &())
            .await?;
        match text.as_str() {
            "0" => Ok(AlternateLimits::Disabled),
            "1" => Ok(AlternateLimits::Enabled),
            _ => Err(Error::BadResponse),
        }
    }

    pub async fn toggle_alt_speed_limits(&self) -> Result<()> {
        self.post("/api/v2/transfer/toggleSpeedLimitsMode", &())
            .await
    }

    //
    // Torrents
    //

    pub async fn get_torrents(&self) -> Result<Vec<Torrent>> {
        self.post_and_decode("/api/v2/torrents/info", &()).await
    }

    pub async fn add_torrent(&self, torrent: &AddTorrent) -> Result<()> {
        self.post("/api/v2/torrents/add", &torrent).await
    }

    //
    // Categories
    //

    pub async fn get_categories(&self) -> Result<HashMap<String, Category>> {
        self.post_and_decode("/api/v2/torrents/categories", &())
            .await
    }

    pub async fn add_category(&self, name: &str, path: &str) -> Result<()> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("category", name);
        form.insert("savePath", path);
        self.post("/api/v2/torrents/createCategory", &form).await
    }

    pub async fn edit_category(&self, name: &str, path: &str) -> Result<()> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("category", name);
        form.insert("savePath", path);
        self.post("/api/v2/torrents/editCategory", &form).await
    }

    pub async fn remove_category(&self, name: &str) -> Result<()> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("categories", name);
        self.post("/api/v2/torrents/removeCategories", &form).await
    }

    //
    // Tags
    //

    pub async fn get_tags(&self) -> Result<HashSet<String>> {
        self.post_and_decode("/api/v2/torrents/tags", &()).await
    }

    pub async fn create_tags(&self, tags: &[String]) -> Result<HashSet<String>> {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("tags", tags.join("|"));
        self.post_and_decode("/api/v2/torrents/createTags", &form)
            .await
    }

    pub async fn delete_tags(&self, tags: &[String]) -> Result<HashSet<String>> {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("tags", tags.join("|"));
        self.post_and_decode("/api/v2/torrents/deleteTags", &form)
            .await
    }
}
