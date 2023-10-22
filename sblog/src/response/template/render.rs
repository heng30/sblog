use super::super::cache;
use super::super::data;
use crate::config;
use std::path::Path;
use tokio::fs;

pub async fn homepage(page: usize) -> String {
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
        .replace("$${{site-logo-tab}}", &webinfo.site_logo_tab)
        .replace("$${{post-title}}", "Home");

    let header = header
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{site-name}}", &webinfo.site_name);

    let body = body.replace(
        "$${{post-summary-list}}",
        render_post_summary(page).await.as_str(),
    );

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
        .replace("$${{site-logo-tab}}", &webinfo.site_logo_tab)
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
    let mut html = String::new();
    let parser = pulldown_cmark::Parser::new(text);
    pulldown_cmark::html::push_html(&mut html, parser);
    return Some(html);
}

async fn render_post_summary(page: usize) -> String {
    const PAGE_STEP: usize = 10;
    let posts = { cache::get_postinfo_all() };
    let mut ps = vec![];

    for (id, pi) in posts.into_iter() {
        let (post_name, post_tag, post_date, _) = {
            match parse_post_path(&pi.path) {
                Some(item) => item,
                _ => continue,
            }
        };

        ps.push(data::PostSummary {
            id,
            name: post_name,
            tag: post_tag,
            date: post_date,
            ..Default::default()
        });
    }

    ps.sort_by(|a, b| b.date.cmp(&a.date));

    let start_index = if page * PAGE_STEP > ps.len() {
        i32::max(0, ps.len() as i32 - PAGE_STEP as i32) as usize
    } else {
        page
    };

    let end_index = usize::min(start_index + PAGE_STEP, ps.len());

    let mut html = String::new();
    let summary_dir = config::conf::monitor().summary;
    for item in ps[start_index..end_index].iter_mut() {
        let mut summary_path = summary_dir.join(item.name.as_str());
        summary_path.set_extension("summary");
        item.text = match fs::read_to_string(&summary_path).await {
            Ok(text) => text,
            _ => String::default(),
        };

        html.push_str(&format!("<article class='article-card'> <h3> <a href='/post/{}'>{}</a> </h3> <p class='article-date'>{}</p> <div class='article-summary'>{} </div> </article>", item.id, item.name, item.date, item.text));
    }

    html
}

pub async fn about() -> String {
    let (frame, header, body) = {
        let ptc = cache::POST_TEMPLATE_CACHE.lock().unwrap();
        (
            ptc.get("frame").unwrap().to_string(),
            ptc.get("header").unwrap().to_string(),
            ptc.get("about-body").unwrap().to_string(),
        )
    };

    let webinfo = config::conf::webinfo();
    let frame = frame
        .replace("$${{site-name}}", &webinfo.site_name)
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{site-logo-tab}}", &webinfo.site_logo_tab)
        .replace("$${{post-title}}", "About");

    let header = header
        .replace("$${{site-logo}}", &webinfo.site_logo)
        .replace("$${{site-name}}", &webinfo.site_name);

    let about_md_path = config::conf::template_dir().join("about-body.md");
    let about_html = match fs::read_to_string(&about_md_path).await {
        Ok(text) => render_md(&text).unwrap_or(text),
        _ => String::default(),
    };

    let body = body.replace("$${{about}}", &about_html);

    frame
        .replace("$${{header}}", &header)
        .replace("$${{body}}", &body)
}
