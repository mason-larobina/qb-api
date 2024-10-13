use crate::api::Api;
use crate::data::*;
use crate::error::Error;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait TorrentData<T> {
    async fn properties(&self, other: &'_ T) -> Result<TorrentProperties, Error>;
    async fn trackers(&self, other: &'_ T) -> Result<Vec<Tracker>, Error>;
    // TOOD: when impl'd on Api, self is 'a when it does not need to be
    // when impl'd on Torrent, other is &Api which also does not need to be 'a
    async fn contents<'a>(&'a self, other: &'a T) -> Result<Vec<TorrentInfo<'a>>, Error>;
}

#[async_trait]
/// Assist in forcing a torrent recheck
pub trait Recheck<T> {
    async fn recheck(&self, other: &'_ T) -> Result<(), Error>;
}

#[async_trait]
/// Assist in setting a category for a torrent
pub trait Category<T> {
    async fn set_category(&self, other: &'_ T, category: &str) -> Result<(), Error>;
}

#[async_trait]
/// Resume a torrent
pub trait Resume<T> {
    async fn resume(&self, other: &'_ T) -> Result<(), Error>;
}

#[async_trait]
/// Pause a torrent
pub trait Pause<T> {
    async fn pause(&self, other: &'_ T) -> Result<(), Error>;
}

#[async_trait]
/// Add a tag to a torrent
pub trait Tags<T> {
    async fn add_tag(&self, other: &'_ T, tags: &[String]) -> Result<(), Error>;
}

#[async_trait]
impl Category<Api> for Torrent {
    async fn set_category(&self, api: &'_ Api, category: &str) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hash.hash.as_str());
        form.insert("category", category);
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/setCategory"))
            .headers(api.headers()?));
        let response = dbg!(request.send().await?);
        let _data = dbg!(response.bytes().await?);
        Ok(())
    }
}

#[async_trait]
impl TorrentData<Api> for Torrent {
    async fn properties(&self, api: &'_ Api) -> Result<TorrentProperties, Error> {
        self.hash.properties(api).await
    }

    async fn trackers(&self, api: &'_ Api) -> Result<Vec<Tracker>, Error> {
        self.hash.trackers(api).await
    }

    async fn contents<'a>(&'a self, api: &'a Api) -> Result<Vec<TorrentInfo<'a>>, Error> {
        self.hash.contents(api).await
    }
}

#[async_trait]
impl TorrentData<Api> for Hash {
    async fn properties(&self, api: &'_ Api) -> Result<TorrentProperties, Error> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash.as_str());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/properties"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let props = serde_json::from_slice(&data)?;
        Ok(props)
    }

    async fn trackers(&self, api: &'_ Api) -> Result<Vec<Tracker>, Error> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash.as_str());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/trackers"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let trackers = serde_json::from_slice(&data)?;
        Ok(trackers)
    }

    async fn contents<'a>(&'a self, api: &'a Api) -> Result<Vec<TorrentInfo<'a>>, Error> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash.as_str());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/files"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        let data = dbg!(response.bytes().await?);
        let info = serde_json::from_slice::<Vec<TorrentInfoSerde>>(&data)?
            .into_iter()
            .map(|x| x.into_info(self))
            .collect();
        Ok(info)
    }
}

#[async_trait]
impl TorrentData<Torrent> for Api {
    async fn properties(&self, torrent: &'_ Torrent) -> Result<TorrentProperties, Error> {
        torrent.hash.properties(self).await
    }

    async fn trackers(&self, torrent: &'_ Torrent) -> Result<Vec<Tracker>, Error> {
        torrent.hash.trackers(self).await
    }

    async fn contents<'a>(&'a self, torrent: &'a Torrent) -> Result<Vec<TorrentInfo<'a>>, Error> {
        torrent.hash.contents(self).await
    }
}

#[async_trait]
impl Resume<Api> for Torrent {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        self.hash.resume(api).await
    }
}

#[async_trait]
impl Resume<Api> for Hash {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hash.as_str());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/resume"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Resume<Api> for Vec<Hash> {
    async fn resume(&self, api: &'_ Api) -> Result<(), Error> {
        let form = Api::hashes_form_data(self.as_ref());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/resume"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Pause<Api> for Torrent {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        self.hash.pause(api).await
    }
}

#[async_trait]
impl Pause<Api> for Hash {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("hash", self.hash.as_str());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/pause"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Pause<Api> for Vec<Hash> {
    async fn pause(&self, api: &'_ Api) -> Result<(), Error> {
        let form = Api::hashes_form_data(self.as_ref());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/pause"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Tags<Api> for Torrent {
    async fn add_tag(&self, api: &'_ Api, tags: &'_ [String]) -> Result<(), Error> {
        self.hash.add_tag(api, tags).await
    }
}

#[async_trait]
impl Tags<Api> for Hash {
    async fn add_tag(&self, api: &'_ Api, tags: &'_ [String]) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hash.clone());
        form.insert("tags", tags.join(","));
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/addTags"))
            .headers(api.headers()?)
            .form(&form));
        let response = dbg!(request.send().await?);
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[async_trait]
impl Recheck<Api> for Hash {
    async fn recheck(&self, api: &'_ Api) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("hashes", self.hash.as_str());
        let request = dbg!(api
            .client
            .post(api.endpoint("/api/v2/torrents/recheck"))
            .headers(api.headers()?)
            .form(&form));
        let _response = dbg!(request.send().await?);
        Ok(())
    }
}

#[async_trait]
impl Recheck<Api> for Torrent {
    async fn recheck(&self, api: &'_ Api) -> Result<(), Error> {
        self.hash.recheck(api).await?;
        Ok(())
    }
}
