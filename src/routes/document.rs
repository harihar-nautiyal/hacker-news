use maud::{DOCTYPE, Markup, html};

pub fn Document(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="/static/tailwind.css" {}
                script src="/static/htmx.js" {}
                script src="/static/ext/hx-preload.js" {}
            }
            body {
                (content)
            }
        }
    }
}
