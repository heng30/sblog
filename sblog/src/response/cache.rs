use super::data::PostInfo;
use crate::config;
use md5;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    static ref POST_INFO_CACHE: Mutex<HashMap<String, PostInfo>> = Mutex::new(HashMap::new());
}

pub fn init() {
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
