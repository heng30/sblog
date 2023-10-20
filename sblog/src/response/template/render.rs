use super::super::cache;
use crate::config;
use std::path::Path;
use tokio::fs;

pub fn homepage() -> String {
    let (frame, header, body) = {
        let ptc = cache::POST_TEMPLATE_CACHE.lock().unwrap();
        (
            ptc.get("frame").unwrap().to_string(),
            ptc.get("header").unwrap().to_string(),
            ptc.get("home-body").unwrap().to_string(),
        )
    };

    let webinfo = config::conf::webinfo();
    let frame = frame
        .replace("$${{site-name}}", &webinfo.site_name)
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{post-title}}", "Home");

    let header = header
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{site-name}}", &webinfo.site_name);

    frame
        .replace("$${{header}}", &header)
        .replace("$${{body}}", &body)
}

pub async fn post(id: &str) -> Option<String> {
    let (post_text, post_path) = match cache::get_postinfo(id) {
        Some(pi) => match fs::read_to_string(&pi.path).await {
            Ok(text) => (text, pi.path.clone()),
            _ => return None,
        },
        _ => return None,
    };

    let mut is_md = true;
    let (post_name, _post_tag, post_date) = {
        let items = Path::new(&post_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split("@@")
            .map(|n| n.clone())
            .collect::<Vec<_>>();

        if items.len() != 3 {
            log::warn!("invalid filename: {}", post_path);
            return None;
        }

        if items[2].to_lowercase().ends_with(".html") {
            is_md = false;
        }

        let post_date = items[2]
            .to_lowercase()
            .replace(".md", "")
            .replace(".html", "");

        log::debug!("{:?}", post_date);

        (items[0], items[1], post_date)
    };

    if is_md {
        // TODO: convert md to html
    }

    let (frame, header, body) = {
        let ptc = cache::POST_TEMPLATE_CACHE.lock().unwrap();
        (
            ptc.get("frame").unwrap().to_string(),
            ptc.get("header").unwrap().to_string(),
            ptc.get("post-body").unwrap().to_string(),
        )
    };

    let webinfo = config::conf::webinfo();
    let frame = frame
        .replace("$${{site-name}}", &webinfo.site_name)
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{post-title}}", "Home");

    let header = header
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{site-name}}", &webinfo.site_name);

    let body = body
        .replace("$${{post-title}}", &post_name)
        .replace("$${{post-date}}", &post_date)
        .replace("$${{post-article}}", &post_text);

    Some(
        frame
            .replace("$${{header}}", &header)
            .replace("$${{body}}", &body),
    )
}
