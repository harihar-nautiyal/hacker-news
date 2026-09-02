use maud::{Markup, PreEscaped, html};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Head<'a> {
    pub title: &'a str,
    pub description: &'a str,
    #[builder(default)]
    pub canonical_url: Option<&'a str>,
    #[builder(default = "website")]
    pub og_type: &'a str,
    #[builder(default)]
    pub structured_data: Option<&'a str>,
}

impl<'a> Head<'a> {
    pub fn render(&self) -> Markup {
        let full_title = if self.title.ends_with("Hacker News") || self.title.ends_with("Hacker News SPA") {
            self.title.to_string()
        } else {
            format!("{} | Hacker News", self.title)
        };

        html! {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            meta name="description" content=(self.description);
            meta name="robots" content="index, follow";
            meta name="theme-color" content="#0a0a0a";
            meta name="color-scheme" content="dark";

            title { (full_title) }

            // Open Graph / Facebook
            meta property="og:site_name" content="Hacker News SPA";
            meta property="og:title" content=(full_title);
            meta property="og:description" content=(self.description);
            meta property="og:type" content=(self.og_type);
            @if let Some(canonical) = self.canonical_url {
                meta property="og:url" content=(canonical);
                link rel="canonical" href=(canonical);
            }

            // Twitter Cards
            meta name="twitter:card" content="summary";
            meta name="twitter:title" content=(full_title);
            meta name="twitter:description" content=(self.description);

            // Favicon
            link rel="icon" type="image/svg+xml" href="/favicon.ico";

            // Deferred client scripts (eliminates render-blocking penalty)
            script src="/static/htmx.js" defer {}
            script src="/static/ext/hx-preload.js" defer {}

            // Preload & Stylesheet
            link rel="stylesheet" href="/static/tailwind.css";

            // Structured Data (JSON-LD)
            @if let Some(json_ld) = self.structured_data {
                script type="application/ld+json" {
                    (PreEscaped(json_ld))
                }
            }
        }
    }
}

pub fn head(
    title: &str,
    description: &str,
    canonical_url: Option<&str>,
    og_type: &str,
    structured_data: Option<&str>,
) -> Markup {
    Head::builder()
        .title(title)
        .description(description)
        .canonical_url(canonical_url)
        .og_type(og_type)
        .structured_data(structured_data)
        .build()
        .render()
}
