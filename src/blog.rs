use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

use anyhow::Ok;
use highlight_pulldown::highlight_with_theme;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

use crate::model::{PostList, Post};
use crate::model::PostMetadata;

pub fn workthrough() -> anyhow::Result<PostList> {
    let mut list = PostList {
        metadata: Vec::new(),
    };

    let path = PathBuf::from("blog/");
    fs::read_dir(path).unwrap()
        .into_iter()
        .filter(|p| {
            let path = p.as_ref().unwrap().path();
            println!(">>>>>>{:?}", &path);

            //fixed: .DS_Store error
            let ext = path.extension();
            ext.is_some() && ext.unwrap() == "md"
        })
        .for_each(|p| {
            let path = p.as_ref().unwrap().path();
            let post_md = read_file_matadata(&path);
            list.metadata.push(post_md.unwrap());
        });

    Ok(list)
}

fn read_file_matadata(path: &PathBuf) -> anyhow::Result<PostMetadata> {
    let pp = path.file_name().unwrap().to_os_string();
    let ppp = String::from(pp.to_string_lossy());

    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut md = PostMetadata::default();
    md.name = ppp.replace(".md", "");

    buf.lines()
        .take_while(|line| is_header(&line.as_ref().unwrap()))
        .for_each(|line| {
            println!("hello");
            let l = line.unwrap().replace("-->", "")[4..].to_lowercase().trim().to_string();
            println!("----{}", &l);

            if l.starts_with("title") {
                md.title = format!("{}", l[6..].trim())
            } else if l.starts_with("date") {
                md.date = Some(chrono::NaiveDate::parse_from_str(l[5..].trim(), "%Y-%m-%d").unwrap())
            } else if l.starts_with("category") {
                md.categories = l[9..].trim().split(",").map(|s| s.trim().to_string()).collect()
            } else if l.starts_with("tag") {
                md.tags = l[4..].trim().split(",").map(|s| s.trim().to_string()).collect()
            }
        });

    Ok(md)
}

/// 是否为元数据
fn is_header(line: &String) -> bool {
    line.starts_with("<!--")
}

///
pub fn open_markdown_file(mid: String) -> anyhow::Result<Post> {
    let path = format!("blog/{}.md", mid);
    let lines = fs::read_to_string(path).unwrap()
        .lines()
        .map(String::from)
        .collect();

    html_from_markdown(&lines)
}

/// 调用pulldown-cmark将markdown文本转换为html文本块
fn html_from_markdown(lines: &Vec<String>) -> anyhow::Result<Post> {

    let mut post = Post::default();

    let mut markdown = String::new();
    for line in lines {
        if is_header(line) {
            let l = line.replace("-->", "")[4..].to_lowercase().trim().to_string();
            println!("{}", &l);

            if l.starts_with("title") {
                post.title = format!("{}", l[6..].trim())
            } else if l.starts_with("date") {
                post.date = Some(chrono::NaiveDate::parse_from_str(l[5..].trim(), "%Y-%m-%d").unwrap())
            } else if l.starts_with("category") {
                post.categories = l[9..].trim().split(",").map(|s| s.trim().to_string()).collect()
            } else if l.starts_with("tag") {
                post.tags = l[4..].trim().split(",").map(|s| s.trim().to_string()).collect()
            }
        } else {
            markdown.push_str(&line);
            markdown.push('\n');
        }
    }

    let parser = Parser::new(&markdown);

    let parser = highlight_with_theme(parser, "base16-ocean.dark").unwrap();

    let mut html_str = String::new();
    html::push_html(&mut html_str, parser.into_iter());

    post.content = html_str;

    Ok(post)
}