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
    let (mut post_text, post_path) = match cache::get_postinfo(id) {
        Some(pi) => match fs::read_to_string(&pi.path).await {
            Ok(text) => (text, pi.path.clone()),
            _ => return None,
        },
        _ => return None,
    };

    let (post_name, post_tag, post_date, is_md) = {
        match parse_post_path(&post_path) {
            Some(item) => item,
            _ => return None,
        }
    };

    let post_tag = render_tag(&post_tag);

    if is_md {
        post_text = match render_md(&post_text) {
            Some(text) => text,
            _ => return None,
        }
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
        .replace("$${{post-tag}}", &post_tag)
        .replace("$${{post-article}}", &post_text);

    Some(
        frame
            .replace("$${{header}}", &header)
            .replace("$${{body}}", &body),
    )
}

pub fn parse_post_path(path: &str) -> Option<(String, String, String, bool)> {
    let items = Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split("@@")
        .map(|n| n.clone())
        .collect::<Vec<_>>();

    if items.len() != 3 {
        log::warn!("invalid filename: {}", path);
        return None;
    }

    let is_md = !items[2].to_lowercase().ends_with(".html");

    let post_date = items[2]
        .to_lowercase()
        .replace(".md", "")
        .replace(".html", "");

    Some((items[0].to_string(), items[1].to_string(), post_date, is_md))
}

fn render_tag(tags: &str) -> String {
    let tag_colors = vec![
        "#CCE0FF", "#BAF3DB", "#C1F0F5", "#DFD8FD", "#FFD2CC", "#FDD0EC", "#D3F1A7", "#DCDFE4",
    ];

    let mut tags_str = String::default();
    for (index, tag) in tags.split(",").enumerate() {
        let span = format!(
            "<span  class='tag-span' style='background: {};'>{}</span>",
            tag_colors[index % tag_colors.len()],
            tag
        );
        tags_str = format!("{}\n{}", tags_str, span);
    }
    tags_str
}

fn render_md(text: &str) -> Option<String> {
    // TODO
    return Some(text.to_string());
}
