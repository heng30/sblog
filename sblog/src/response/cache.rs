use super::{data::PostInfo, template};
use crate::config;
use md5;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    static ref POST_INFO_CACHE: Mutex<HashMap<String, PostInfo>> = Mutex::new(HashMap::new());
}

pub fn init() {
    init_template();
    init_postinfo();
}

fn init_postinfo() {
    let path = config::conf::monitor().md;
    let metadata = fs::metadata(&path).unwrap();

    if metadata.is_dir() {
        for entry in fs::read_dir(&path).unwrap().flatten() {
            let path = entry.path().as_path().to_str().unwrap().to_string();

            if !(path.to_lowercase().ends_with(".html") || path.to_lowercase().ends_with("md")) {
                continue;
            }

            let id = format!("{:x}", md5::compute(&path));
            log::debug!("{}, {}", id, path);
            {
                POST_INFO_CACHE
                    .lock()
                    .unwrap()
                    .insert(id, PostInfo { path });
            }
        }
    } else {
        panic!("cache init panic");
    }
}

pub fn get_postinfo_count() -> usize {
    POST_INFO_CACHE.lock().unwrap().len()
}

pub fn get_postinfo(id: &str) -> Option<PostInfo> {
    POST_INFO_CACHE.lock().unwrap().get(id).cloned()
}

pub fn get_postinfo_all() -> Vec<(String, PostInfo)> {
    POST_INFO_CACHE
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

pub fn add_postinfo(id: String, pi: PostInfo) {
    POST_INFO_CACHE.lock().unwrap().insert(id, pi);
}

pub fn remove_postinfo(id: &str) {
    POST_INFO_CACHE.lock().unwrap().remove(id);
}

#[allow(unused)]
pub fn is_postinfo_exist(id: &str) -> bool {
    POST_INFO_CACHE.lock().unwrap().contains_key(id)
}

pub fn update_postinfo_path(from_id: String, to_id: String, to_path: String) {
    let pi = { POST_INFO_CACHE.lock().unwrap().remove(&from_id) };

    if let Some(mut v) = pi {
        v.path = to_path;
        POST_INFO_CACHE.lock().unwrap().insert(to_id, v);
    } else {
        POST_INFO_CACHE
            .lock()
            .unwrap()
            .insert(to_id, PostInfo { path: to_path });
    }
}

lazy_static! {
    pub static ref POST_TEMPLATE_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn init_template() {
    let template_dir = config::conf::template_dir();
    let frame = template_dir.join("frame.html");
    let header = template_dir.join("header.html");
    let home_body = template_dir.join("home-body.html");
    let post_body = template_dir.join("post-body.html");
    let about_body = template_dir.join("about-body.html");
    let about_body_md = template_dir.join("about-body.md");

    let mut ptc = POST_TEMPLATE_CACHE.lock().unwrap();
    match fs::read_to_string(&frame) {
        Ok(text) => {
            ptc.insert("frame".to_string(), text);
        }
        _ => {
            fs::write(&frame, template::frame::TEMPLATE).unwrap();
            ptc.insert("frame".to_string(), template::frame::TEMPLATE.to_string());
        }
    }

    match fs::read_to_string(&header) {
        Ok(text) => {
            ptc.insert("header".to_string(), text);
        }
        _ => {
            fs::write(&header, template::header::TEMPLATE).unwrap();
            ptc.insert("header".to_string(), template::header::TEMPLATE.to_string());
        }
    }

    match fs::read_to_string(&home_body) {
        Ok(text) => {
            ptc.insert("home-body".to_string(), text);
        }
        _ => {
            fs::write(&home_body, template::home_body::TEMPLATE).unwrap();
            ptc.insert(
                "home-body".to_string(),
                template::home_body::TEMPLATE.to_string(),
            );
        }
    }

    match fs::read_to_string(&post_body) {
        Ok(text) => {
            ptc.insert("post-body".to_string(), text);
        }
        _ => {
            fs::write(&post_body, template::post_body::TEMPLATE).unwrap();
            ptc.insert(
                "post-body".to_string(),
                template::post_body::TEMPLATE.to_string(),
            );
        }
    }

    match fs::read_to_string(&about_body) {
        Ok(text) => {
            ptc.insert("about-body".to_string(), text);
        }
        _ => {
            fs::write(&about_body, template::about_body::TEMPLATE).unwrap();
            ptc.insert(
                "about-body".to_string(),
                template::about_body::TEMPLATE.to_string(),
            );
        }
    }

    match fs::read_to_string(&about_body_md) {
        Ok(_) => {}
        _ => {
            fs::write(&about_body_md, "").unwrap();
        }
    }
}
