// Copyright (C) 2023 Enrico Guiraud
//
// 该文件来源于highlight-pulldown，因为需要修改一些代码，所以进行了复制修改
// 同时也复制修改了syntect的一些源代码

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Theme, ThemeSet};
use syntect::html::{append_highlighted_html_for_styled_line, IncludeBackground};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("theme '{0}' is not avaiable")]
    InvalidTheme(String),
    #[error("error highlighting code")]
    HighlightError(#[from] syntect::Error),
}

pub struct PulldownHighlighter {
    syntaxset: SyntaxSet,
    themeset: ThemeSet,
    theme: String,
}

/// A highlighter that can be instantiated once and used many times for better performance.
impl PulldownHighlighter {
    pub fn new(theme: &str) -> Result<PulldownHighlighter, Error> {
        let syntaxset = SyntaxSet::load_defaults_newlines();
        let themeset = ThemeSet::load_defaults();

        // check that the theme exists
        themeset
            .themes
            .get(theme)
            .ok_or(Error::InvalidTheme(theme.to_string()))?;

        Ok(PulldownHighlighter {
            syntaxset,
            themeset,
            theme: theme.to_string(),
        })
    }

    pub fn highlight<'a, It>(&self, events: It) -> Result<Vec<Event<'a>>, Error>
        where
            It: Iterator<Item = Event<'a>>,
    {
        let mut in_code_block = false;

        let mut syntax = self.syntaxset.find_syntax_plain_text();

        let theme = self
            .themeset
            .themes
            .get(&self.theme)
            .ok_or(Error::InvalidTheme(self.theme.clone()))?;

        let mut to_highlight = String::new();
        let mut out_events = Vec::new();

        for event in events {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    match kind {
                        CodeBlockKind::Fenced(lang) => {
                            syntax = self.syntaxset.find_syntax_by_token(&lang).unwrap_or(syntax)
                        }
                        CodeBlockKind::Indented => {}
                    }
                    in_code_block = true;
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if !in_code_block {
                        panic!("this should never happen");
                    }
                    let html =
                        highlighted_html_for_string(&to_highlight, &self.syntaxset, syntax, theme)?;

                    to_highlight.clear();
                    in_code_block = false;
                    out_events.push(Event::Html(CowStr::from(html)));
                }
                Event::Text(t) => {
                    if in_code_block {
                        to_highlight.push_str(&t);
                    } else {
                        out_events.push(Event::Text(t));
                    }
                }
                e => {
                    out_events.push(e);
                }
            }
        }

        Ok(out_events)
    }
}

pub fn highlight_with_theme<'a, It>(events: It, theme: &str) -> Result<Vec<Event<'a>>, Error>
    where
        It: Iterator<Item = Event<'a>>,
{
    let highlighter = PulldownHighlighter::new(theme)?;
    highlighter.highlight(events)
}

pub fn highlighted_html_for_string(
    s: &str,
    ss: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &Theme,
) -> Result<String, Error> {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let (mut output, bg) = start_highlighted_html_snippet(theme);

    for line in LinesWithEndings::from(s) {
        let regions = highlighter.highlight_line(line, ss)?;
        append_highlighted_html_for_styled_line(
            &regions[..],
            IncludeBackground::IfDifferent(bg),
            &mut output,
        )?;
    }
    output.push_str("</code></pre>\n");
    Ok(output)
}

fn start_highlighted_html_snippet(t: &Theme) -> (String, Color) {
    let c = t.settings.background.unwrap_or(Color::WHITE);
    (
        format!(
            "<pre>\n<code style=\"background-color:#{:02x}{:02x}{:02x};\">",
            c.r, c.g, c.b
        ),
        c,
    )
}