use super::{cache, data::PostInfo};
use crate::config;
use md5;
use notify::{
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    Event, EventKind, RecursiveMode, Result, Watcher,
};
use tokio::task;
use tokio::time::{sleep, Duration};

pub fn init() {
    task::spawn(async move {
        let mut watcher = notify::recommended_watcher(|res: Result<Event>| match res {
            Ok(event) => match event.kind {
                EventKind::Create(CreateKind::File) => {
                    for path in event.paths.into_iter() {
                        let path = path.to_str().unwrap().to_string();
                        let id = format!("{:X}", md5::compute(&path));

                        log::debug!("create: {}: {}", id, path);
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

                        log::debug!("rename: {}: {} -> {}: {}", from_id, from_path, to_id, to_path);
                        update_postinfo_path(from_id, to_id, to_path);
                    }
                }
                EventKind::Remove(RemoveKind::File) => {
                    for path in event.paths.into_iter() {
                        let path = path.to_str().unwrap().to_string();
                        let id = format!("{:X}", md5::compute(&path));

                        log::debug!("remove: {}: {}", id, path);
                        remove_postinfo(id, path);
                    }
                }
                _ => (),
            },
            Err(e) => log::warn!("watch error: {:?}", e),
        })
        .unwrap();

        watcher
            .watch(&config::conf::monitor().md, RecursiveMode::NonRecursive)
            .unwrap();

        sleep(Duration::from_secs(60)).await;
    });
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
