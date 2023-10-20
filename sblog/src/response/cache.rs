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
            let id = format!("{:X}", md5::compute(&path));
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

#[allow(unused)]
pub fn get_postinfo(id: &str) -> Option<PostInfo> {
    POST_INFO_CACHE
        .lock()
        .unwrap()
        .get(id)
        .map_or(None, |v| Some(v.clone()))
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

    match pi {
        Some(mut v) => {
            v.path = to_path;
            POST_INFO_CACHE.lock().unwrap().insert(to_id, v);
        }

        _ => (),
    }
}

lazy_static! {
    pub static ref POST_TEMPLATE_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn init_template() {
    let template_dir = config::conf::template_dir();
    let frame = template_dir.join("frame.html");
    let head = template_dir.join("head.html");
    let foot = template_dir.join("foot.html");
    let side = template_dir.join("side.html");

    let home_body = template_dir.join("home-body.html");
    let post_title = template_dir.join("post-title.html");
    let post_body = template_dir.join("post-body.html");

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

    match fs::read_to_string(&head) {
        Ok(text) => {
            ptc.insert("head".to_string(), text);
        }
        _ => {
            fs::write(&head, template::head::TEMPLATE).unwrap();
            ptc.insert("head".to_string(), template::head::TEMPLATE.to_string());
        }
    }

    match fs::read_to_string(&foot) {
        Ok(text) => {
            ptc.insert("foot".to_string(), text);
        }
        _ => {
            fs::write(&foot, template::foot::TEMPLATE).unwrap();
            ptc.insert("foot".to_string(), template::foot::TEMPLATE.to_string());
        }
    }

    match fs::read_to_string(&side) {
        Ok(text) => {
            ptc.insert("side".to_string(), text);
        }
        _ => {
            fs::write(&side, template::side::TEMPLATE).unwrap();
            ptc.insert("side".to_string(), template::side::TEMPLATE.to_string());
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

    match fs::read_to_string(&post_title) {
        Ok(text) => {
            ptc.insert("post-title".to_string(), text);
        }
        _ => {
            fs::write(&post_title, template::post_title::TEMPLATE).unwrap();
            ptc.insert(
                "post-title".to_string(),
                template::post_title::TEMPLATE.to_string(),
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
}
