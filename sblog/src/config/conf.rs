use super::data::{self, Config};
use crate::CResult;
use anyhow::anyhow;
use log::debug;
use platform_dirs::AppDirs;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

const APP_NANME: &str = "sblog";
const DB_NAME: &str = "sblog.db";
const CONFIG_NAME: &str = "sblog.conf";

lazy_static! {
    pub static ref CONFIG: Mutex<RefCell<Config>> = Mutex::new(RefCell::new(Config::default()));
}

pub fn init() {
    if let Err(e) = CONFIG.lock().unwrap().borrow_mut().init() {
        panic!("{:?}", e);
    }
}

pub fn server() -> data::Server {
    CONFIG.lock().unwrap().borrow().server.clone()
}

pub fn monitor() -> data::Monitor {
    CONFIG.lock().unwrap().borrow().monitor.clone()
}

pub fn webinfo() -> data::WebInfo {
    CONFIG.lock().unwrap().borrow().webinfo.clone()
}

pub fn rssinfo() -> data::RssInfo {
    CONFIG.lock().unwrap().borrow().rssinfo.clone()
}

pub fn template_dir() -> PathBuf {
    let conf = CONFIG.lock().unwrap();
    let conf = conf.borrow();
    conf.template_dir.clone()
}

pub fn assest_dir() -> PathBuf {
    let conf = CONFIG.lock().unwrap();
    let conf = conf.borrow();
    conf.assest_dir.clone()
}

#[allow(unused)]
pub fn config() -> data::Config {
    CONFIG.lock().unwrap().borrow().clone()
}

#[allow(unused)]
pub fn save(conf: data::Config) -> CResult {
    let config = CONFIG.lock().unwrap();
    let mut config = config.borrow_mut();
    *config = conf;
    config.save()
}

impl Config {
    pub fn init(&mut self) -> CResult {
        let app_dirs = AppDirs::new(Some(APP_NANME), true).unwrap();
        Self::init_app_dir(&app_dirs)?;
        self.init_config(&app_dirs)?;
        self.init_path()?;
        self.load()?;
        debug!("{:?}", self);
        Ok(())
    }

    fn init_app_dir(app_dirs: &AppDirs) -> CResult {
        fs::create_dir_all(&app_dirs.data_dir)?;
        fs::create_dir_all(&app_dirs.config_dir)?;
        Ok(())
    }

    fn init_path(&self) -> CResult {
        fs::create_dir_all(&self.template_dir)?;
        fs::create_dir_all(&self.assest_dir)?;
        fs::create_dir_all(&self.monitor.post)?;
        fs::create_dir_all(&self.monitor.md)?;
        fs::create_dir_all(&self.monitor.assest)?;
        fs::create_dir_all(&self.monitor.summary)?;
        Ok(())
    }

    fn init_config(&mut self, app_dirs: &AppDirs) -> CResult {
        self.config_path = app_dirs.config_dir.join(CONFIG_NAME);
        self.db_path = app_dirs.data_dir.join(DB_NAME);

        self.template_dir = app_dirs.data_dir.join("template");
        self.assest_dir = app_dirs.data_dir.join("assest");

        self.monitor.post = app_dirs.data_dir.join("post");
        self.monitor.md = self.monitor.post.join("md");
        self.monitor.assest = self.monitor.post.join("assest");
        self.monitor.summary = self.monitor.post.join("summary");

        Ok(())
    }

    fn load(&mut self) -> CResult {
        match fs::read_to_string(&self.config_path) {
            Ok(text) => match serde_json::from_str::<Config>(&text) {
                Ok(c) => {
                    self.server = c.server;
                    self.webinfo = c.webinfo;
                    self.rssinfo = c.rssinfo;
                    Ok(())
                }
                Err(e) => Err(anyhow!("{:?}", e).into()),
            },
            Err(_) => match serde_json::to_string_pretty(self) {
                Ok(text) => Ok(fs::write(&self.config_path, text)?),
                Err(e) => Err(anyhow!("{:?}", e).into()),
            },
        }
    }

    #[allow(unused)]
    pub fn save(&self) -> CResult {
        match serde_json::to_string_pretty(self) {
            Ok(text) => Ok(fs::write(&self.config_path, text)?),
            Err(e) => Err(anyhow!("{:?}", e).into()),
        }
    }
}
