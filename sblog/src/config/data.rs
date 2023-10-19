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
    pub assest: PathBuf,
}
