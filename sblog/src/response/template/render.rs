use super::super::cache;
use super::super::data;
use crate::config;
use atom_syndication::FixedDateTime;
use atom_syndication::{Category, Entry, FeedBuilder, Link, Person, Text};
use std::path::Path;
use std::str::FromStr;
use tokio::fs;

const PAGE_STEP: usize = 8;
const RSS_COUNT: usize = 10;

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

    let (prev_page, current_page, total_page, next_page) = get_page_info(page);
    let body = body
        .replace("$${{prev-page}}", &format!("{}", prev_page))
        .replace("$${{current-page}}", &format!("{}", current_page))
        .replace("$${{total-page}}", &format!("{}", total_page))
        .replace("$${{next-page}}", &format!("{}", next_page))
        .replace(
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

#[allow(clippy::map_clone)]
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
    let tag_colors = [
        "#CCE0FF", "#BAF3DB", "#C1F0F5", "#DFD8FD", "#FFD2CC", "#FDD0EC", "#D3F1A7", "#DCDFE4",
    ];

    let mut tags_str = String::default();
    for (index, tag) in tags.split(',').enumerate() {
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
    Some(html)
}

async fn render_post_summary(page: usize) -> String {
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

    let start_index = if page * PAGE_STEP >= ps.len() {
        if 0 == ps.len() % PAGE_STEP {
            usize::max(0, ps.len() - PAGE_STEP)
        } else {
            ps.len() - ps.len() % PAGE_STEP
        }
    } else {
        page * PAGE_STEP
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

fn get_page_info(page: usize) -> (usize, usize, usize, usize) {
    let total_page = f64::ceil(cache::get_postinfo_count() as f64 / PAGE_STEP as f64) as usize;

    let current_page = usize::min(page + 1, total_page);

    let prev_page = if current_page > 1 {
        current_page - 2
    } else {
        0
    };

    let next_page = current_page;

    (prev_page, current_page, total_page, next_page)
}

pub async fn rss() -> String {
    let site_name = config::conf::webinfo().site_name;
    let rssinfo = config::conf::rssinfo();

    let mut person = Person::default();
    person.set_name(rssinfo.author.as_str());
    person.set_email(rssinfo.email);
    person.set_uri(rssinfo.uri.clone());

    let mut link = Link::default();
    link.set_href(rssinfo.uri.clone());

    let posts = get_rss_posts().await;

    let updated = FixedDateTime::from_str(
        format!(
            "{}T00:00:00-00:00",
            posts.first().unwrap_or(&data::PostSummary::default()).date
        )
        .as_str(),
    )
    .unwrap_or(FixedDateTime::from_str("2000-06-01T15:15:44-05:00").unwrap());

    let mut entry_person = Person::default();
    entry_person.set_name(rssinfo.author.as_str());

    let entry_items = posts
        .into_iter()
        .map(|item| {
            let date = FixedDateTime::from_str(format!("{}T00:00:00-00:00", item.date).as_str())
                .unwrap_or(FixedDateTime::from_str("2000-06-01T15:15:44-05:00").unwrap());

            let mut en = Entry::default();
            en.set_title(item.name);
            en.set_authors(vec![entry_person.clone()]);
            en.set_published(date);
            en.set_updated(date);

            let mut link = Link::default();
            link.set_href(format!("{}/post/{}", rssinfo.uri, item.id));
            en.set_links(vec![link]);

            let mut labels = vec![];
            for tag in item.tag.split(',') {
                let mut cate = Category::default();
                cate.set_label(tag.to_string());
                labels.push(cate);
            }
            en.set_categories(labels);

            en.set_summary(Text::from(item.text));
            en
        })
        .collect::<Vec<_>>();

    FeedBuilder::default()
        .title(site_name.as_str())
        .author(person)
        .link(link)
        .updated(updated)
        .entries(entry_items)
        .build()
        .to_string()
}

async fn get_rss_posts() -> Vec<data::PostSummary> {
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

    if ps.len() > RSS_COUNT {
        ps.drain(RSS_COUNT..);
    }

    let summary_dir = config::conf::monitor().summary;
    for item in ps.iter_mut() {
        let mut summary_path = summary_dir.join(item.name.as_str());
        summary_path.set_extension("summary");
        item.text = match fs::read_to_string(&summary_path).await {
            Ok(text) => text,
            _ => String::default(),
        };
    }

    ps
}
