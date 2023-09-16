use std::cmp::Ordering;
use std::fs;
use std::fs::{DirEntry, File, metadata};
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use std::sync::MutexGuard;
use std::time::SystemTime;
use str_utils::StartsWithIgnoreAsciiCase;

use anyhow::Ok;
use highlight_pulldown::highlight_with_theme;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

use crate::model::{Cache, Post, PostMetadata};

/// 判断是否需要重新加载所有的Markdown文件元数据
/// 根据Markdown文件的创建时间（文件的属性，不需要加载并解析文件获得）与参数时间进行比对
/// 如果创建时间 > 参数时间，则需要重新加载；否则不需要
pub fn need_reload(date: SystemTime) -> bool {

    let entry = fs::read_dir(PathBuf::from("blog")).unwrap()
        .into_iter()
        .filter(|dir| is_markdown(dir.as_ref().unwrap()))
        .max_by(|x,y| file_created_order(x.as_ref().unwrap(), y.as_ref().unwrap()));

    match entry {
        None => false,
        Some(ent) => {
            let lasted = fs::metadata(ent.unwrap().path()).unwrap().created().unwrap();
            lasted.ge(&date)
        }
    }
}

pub fn work_through(cache: &mut MutexGuard<Cache>) {
    let mut metadata = Vec::new();

    let mut lasted: Option<SystemTime> = None;

    fs::read_dir(PathBuf::from("blog")).unwrap()
        .into_iter()
        .filter(|dir| is_markdown(dir.as_ref().unwrap()))
        .for_each(|p| {
            let path = p.as_ref().unwrap().path();
            let post_md = read_file_matadata(&path).unwrap();

            for cate in &post_md.categories {
                cache.category_cloud.insert(cate.clone());
            }

            for tag in &post_md.tags {
                cache.tag_cloud.insert(tag.clone());
            }

            let md2 = post_md.clone();
            cache.metadata.insert(md2.name.clone(), md2);

            metadata.push(post_md);

            let created = fs::metadata(&path).unwrap().created().unwrap();
            match lasted {
                None => {lasted = Some(created) },
                Some(lasted2) => { if created > lasted2 {lasted = Some(created)}},
            }
        });

    cache.lasted_update = lasted;
}

fn read_file_matadata(path: &PathBuf) -> anyhow::Result<PostMetadata> {
    let pp = path.file_name().unwrap().to_os_string();
    let ppp = String::from(pp.to_string_lossy());

    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut md = PostMetadata::default();
    md.name = ppp.replace(".md", "");

    buf.lines()
        .take_while(|line|
            is_header(&line.as_ref().unwrap()))
        .for_each(|line| {

            let l = line.unwrap()
                .replace("-->", "")[4..]
                .trim()
                .to_string();
            println!("----{}", &l);

            if l.starts_with_ignore_ascii_case("title") {
                md.title = format!("{}", l[6..].trim())
            } else if l.starts_with_ignore_ascii_case("date") {
                md.date = Some(chrono::NaiveDate::parse_from_str(l[5..].trim(), "%Y-%m-%d").unwrap())
            } else if l.starts_with_ignore_ascii_case("category") {
                md.categories = l[9..].trim().split(",").map(|s| s.trim().to_string()).collect()
            } else if l.starts_with_ignore_ascii_case("tag") {
                md.tags = l[4..].trim().split(",").map(|s| s.trim().to_string()).collect()
            }
        });

    Ok(md)
}

/// 是否为元数据
fn is_header(line: &String) -> bool {
    line.starts_with("<!--")
}

fn is_markdown(dir: &DirEntry) -> bool {
    let path = dir.path();
    //fixed: .DS_Store error
    let ext = path.extension();
    ext.is_some() && ext.unwrap() == "md"
}

pub fn file_created_order(a: &DirEntry, b: &DirEntry) -> Ordering {
    let path_a = a.path();
    let path_b = b.path();

    let date_a = fs::metadata(path_a).unwrap().created().unwrap();
    let date_b = fs::metadata(path_b).unwrap().created().unwrap();

    date_a.cmp(&date_b)
}

///
pub fn open_markdown_file(mid: String, metadata: PostMetadata) -> anyhow::Result<Post> {
    let path = format!("blog/{}.md", mid);
    let lines = fs::read_to_string(path).unwrap()
        .lines()
        .map(String::from)
        .collect();

    html_from_markdown(metadata, &lines)
}

/// 调用pulldown-cmark将markdown文本转换为html文本块
fn html_from_markdown(metadata: PostMetadata, lines: &Vec<String>) -> anyhow::Result<Post> {
    let mut post = Post::from(metadata);

    let mut markdown = String::new();
    for line in lines {
        markdown.push_str(&line);
        markdown.push('\n');
    }

    let parser = Parser::new(&markdown);
    let events = highlight_with_theme(parser, "base16-ocean.dark").unwrap();

    let mut html_str = String::new();
    html::push_html(&mut html_str, events.into_iter());

    post.content = html_str;

    Ok(post)
}