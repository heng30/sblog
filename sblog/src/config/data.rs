use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,

    #[serde(skip)]
    pub db_path: PathBuf,

    #[serde(skip)]
    pub template_dir: PathBuf,

    #[serde(skip)]
    pub assest_dir: PathBuf,

    #[serde(skip)]
    pub monitor: Monitor,

    pub server: Server,
    pub webinfo: WebInfo,
    pub rssinfo: RssInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub listen_address: String,
    pub listen_port: u16,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".to_string(),
            listen_port: 8080,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Monitor {
    pub post: PathBuf,
    pub md: PathBuf,
    pub summary: PathBuf,
    pub assest: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebInfo {
    pub site_logo_tab: String,
    pub site_logo: String,
    pub site_name: String,
}

impl Default for WebInfo {
    fn default() -> Self {
        Self {
            site_logo_tab: "/assest/logo.png".to_string(),
            site_logo: "/assest/logo.png".to_string(),
            site_name: "site-name".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RssInfo {
    pub author: String,
    pub email: String,
    pub uri: String,
}

impl Default for RssInfo {
    fn default() -> Self {
        Self {
            author: "author".to_string(),
            email: "author.com".to_string(),
            uri: "https://example.com".to_string(),
        }
    }
}
