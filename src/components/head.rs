use maud::{Markup, html};

pub fn head(page_title: &str) -> Markup {
    html! {
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        title { (page_title) " - Hacker News" }

        script src="/static/htmx.js" {}
        script src="/static/ext/hx-preload.js" {}
        link rel="stylesheet" href="/static/tailwind.css";
    }
}
