use super::{
    cache::{self, POST_TEMPLATE_CACHE},
    data::PostInfo,
};
use crate::config;
use md5;
use notify::{
    event::{CreateKind, DataChange, ModifyKind, RemoveKind, RenameMode},
    Event, EventKind, RecursiveMode, Result, Watcher,
};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;

pub fn init() {
    watcher_md();
    watcher_template();
}

fn is_exclude_file(path: &Path) -> bool {
    path.file_name().unwrap().to_str().unwrap().starts_with(".")
}

fn is_md(path: &Path) -> bool {
    path.extension().unwrap_or(&OsStr::new("")).to_ascii_lowercase() == "md"
}

fn watcher_md() {
    let mut watcher = Box::new(
        notify::recommended_watcher(|res: Result<Event>| match res {
            Ok(event) => match event.kind {
                EventKind::Create(CreateKind::File) => {
                    for path in event.paths.into_iter() {
                        if is_exclude_file(&path) || !is_md(&path) {
                            continue;
                        }

                        let path = path.to_str().unwrap().to_string();
                        let id = format!("{:X}", md5::compute(&path));

                        log::debug!("create: ({}, {})", id, path);
                        add_postinfo(id, path);
                    }
                }
                EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
                    if event.paths.len() != 2 {
                        log::warn!("rename error: {:?}", event);
                    } else {
                        let from_path = event.paths[0].to_str().unwrap().to_string();
                        let to_path = event.paths[1].to_str().unwrap().to_string();
                        let from_id = format!("{:X}", md5::compute(&from_path));
                        let to_id = format!("{:X}", md5::compute(&to_path));

                        log::debug!(
                            "rename: ({}, {}) -> ({}, {})",
                            from_id,
                            from_path,
                            to_id,
                            to_path
                        );
                        update_postinfo_path(from_id, to_id, to_path);
                    }
                }
                EventKind::Remove(RemoveKind::File) => {
                    for path in event.paths.into_iter() {
                        if is_exclude_file(&path) || !is_md(&path) {
                            continue;
                        }

                        let path = path.to_str().unwrap().to_string();
                        let id = format!("{:X}", md5::compute(&path));

                        log::debug!("remove: ({}, {})", id, path);
                        remove_postinfo(id, path);
                    }
                }
                _ => (),
            },
            Err(e) => log::warn!("watch error: {:?}", e),
        })
        .unwrap(),
    );

    watcher
        .watch(&config::conf::monitor().md, RecursiveMode::NonRecursive)
        .unwrap();

    Box::leak(watcher);
}

fn add_postinfo(id: String, path: String) {
    cache::add_postinfo(id, PostInfo { path });
}

fn update_postinfo_path(from_id: String, to_id: String, to_path: String) {
    cache::update_postinfo_path(from_id, to_id, to_path);
}

fn remove_postinfo(id: String, _path: String) {
    cache::remove_postinfo(&id);
}

fn watcher_template() {
    let mut watcher = Box::new(
        notify::recommended_watcher(|res: Result<Event>| match res {
            Ok(event) => match event.kind {
                EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
                    log::debug!("modify: {:?}", event.paths);
                    update_template(event.paths);
                }
                _ => {
                    // log::debug!("{:?}", event);
                }
            },
            Err(e) => log::warn!("watch error: {:?}", e),
        })
        .unwrap(),
    );

    watcher
        .watch(&config::conf::template_dir(), RecursiveMode::NonRecursive)
        .unwrap();

    Box::leak(watcher);
}

fn update_template(paths: Vec<PathBuf>) {
    for path in paths.into_iter() {
        let filename = path.file_name().unwrap().to_str().unwrap();
        if !filename.ends_with(".html") {
            continue;
        }

        let filename = filename.trim_end_matches(".html");
        let text = fs::read_to_string(&path).unwrap();
        let mut ptc = POST_TEMPLATE_CACHE.lock().unwrap();
        ptc.insert(filename.to_string(), text);
    }
}
