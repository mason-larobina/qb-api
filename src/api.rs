use crate::data::{AlternateLimits, BuildInfo, Category, GlobalTransferInfo, Log, Torrent};
use crate::error::{Error, Result};
use crate::queries::{AddTorrent, LogRequest};
use log::*;
use reqwest::{
    header::{HeaderMap, SET_COOKIE},
    Response,
};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::{HashMap, HashSet};
use url::Url;

/// Main handle and access point to working with qbittorrent.
///
/// Full documentation on provided methods is available
/// [here](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1))
#[derive(Debug)]
pub struct Api {
    pub(crate) url: Url,
    pub(crate) headers: HeaderMap,
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
        headers.insert("referer", url.as_str().parse()?);

        let mut api = Self {
            url,
            headers,
            client,
        };

        let response = api.post("/api/v2/auth/login", form).await?;

        for cookie in response.headers().get_all(SET_COOKIE) {
            let cookie = cookie.to_str()?;
            if cookie.starts_with("SID=") {
                let sid_cookie = cookie.split(";").next().unwrap();
                api.headers.insert("cookie", sid_cookie.parse()?);
                debug!("{:?}", api);
                return Ok(api);
            }
        }

        Err(Error::MissingCookie)
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
    // Internal post request functions and utils.
    //

    pub(crate) async fn post<F: Serialize + ?Sized>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<Response> {
        let mut url = self.url.clone();
        url.set_path(path);
        let request = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .form(form);
        debug!("POST -> {:?} {:?}", path, request);
        let response = request.send().await?;
        debug!("POST <- {:?} {:?}", path, response);
        Ok(response)
    }

    pub(crate) async fn post_status<F: Serialize + ?Sized>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<()> {
        let response = self.post(path, form).await?;
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub(crate) async fn post_decode<F: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<T> {
        let response = self.post(path, form).await?;
        let data = response.bytes().await?;
        debug!("POST <- {:?} DATA {:?}", path, std::str::from_utf8(&data));
        let ret = serde_json::from_slice(&data)?;
        Ok(ret)
    }

    pub(crate) async fn post_text<F: Serialize + ?Sized>(
        &self,
        path: &str,
        form: &F,
    ) -> Result<String> {
        let response = self.post(path, form).await?;
        let text = response.text().await?;
        debug!("POST <- {:?} TEXT {:?}", path, text);
        Ok(text)
    }

    //
    // Application info / control
    //

    pub async fn get_app_version(&self) -> Result<String> {
        self.post_text("/api/v2/app/version", &()).await
    }

    pub async fn get_api_version(&self) -> Result<String> {
        self.post_text("/api/v2/app/webapiVersion", &()).await
    }

    pub async fn get_build_info(&self) -> Result<BuildInfo> {
        self.post_decode("/api/v2/app/buildInfo", &()).await
    }

    pub async fn get_default_save_path(&self) -> Result<String> {
        self.post_text("/api/v2/app/defaultSavePath", &()).await
    }

    pub async fn get_main_logs(&self, logs: &LogRequest) -> Result<Vec<Log>> {
        self.post_decode("/api/v2/log/main", &logs).await
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.post_status("/api/v2/app/shutdown", &()).await
    }

    //
    // Speed info / limits / control
    //

    pub async fn get_global_transfer_info(&self) -> Result<GlobalTransferInfo> {
        self.post_decode("/api/v2/transfer/info", &()).await
    }

    pub async fn get_alt_speed_limits_state(&self) -> Result<AlternateLimits> {
        let text = self
            .post_text("/api/v2/transfer/speedLimitsMode", &())
            .await?;
        match text.as_str() {
            "0" => Ok(AlternateLimits::Disabled),
            "1" => Ok(AlternateLimits::Enabled),
            _ => Err(Error::BadResponse),
        }
    }

    pub async fn toggle_alt_speed_limits(&self) -> Result<()> {
        self.post_status("/api/v2/transfer/toggleSpeedLimitsMode", &())
            .await
    }

    //
    // Torrents
    //

    pub async fn get_torrents(&self) -> Result<Vec<Torrent>> {
        self.post_decode("/api/v2/torrents/info", &()).await
    }

    pub async fn add_torrent(&self, torrent: &AddTorrent) -> Result<()> {
        self.post_status("/api/v2/torrents/add", &torrent).await
    }

    //
    // Categories
    //

    pub async fn get_categories(&self) -> Result<HashMap<String, Category>> {
        self.post_decode("/api/v2/torrents/categories", &()).await
    }

    pub async fn add_category(&self, name: &str, path: &str) -> Result<()> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("category", name);
        form.insert("savePath", path);
        self.post_status("/api/v2/torrents/createCategory", &form)
            .await
    }

    pub async fn edit_category(&self, name: &str, path: &str) -> Result<()> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("category", name);
        form.insert("savePath", path);
        self.post_status("/api/v2/torrents/editCategory", &form)
            .await
    }

    pub async fn remove_category(&self, name: &str) -> Result<()> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("categories", name);
        self.post_status("/api/v2/torrents/removeCategories", &form)
            .await
    }

    //
    // Tags
    //

    pub async fn get_tags(&self) -> Result<HashSet<String>> {
        self.post_decode("/api/v2/torrents/tags", &()).await
    }

    fn join_tags<T>(tags: T) -> String
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let mut ret = String::new();
        for (i, tag) in tags.into_iter().enumerate() {
            if i > 0 {
                ret.push(',');
            }
            ret.push_str(tag.as_ref());
        }
        ret
    }

    pub async fn create_tags<T>(&self, tags: T) -> Result<HashSet<String>>
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("tags", Self::join_tags(tags));
        self.post_decode("/api/v2/torrents/createTags", &form).await
    }

    pub async fn delete_tags<T>(&self, tags: T) -> Result<HashSet<String>>
    where
        T: IntoIterator,
        T::Item: AsRef<str>,
    {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("tags", Self::join_tags(tags));
        self.post_decode("/api/v2/torrents/deleteTags", &form).await
    }
}
