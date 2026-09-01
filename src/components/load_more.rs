use maud::{Markup, html};

use crate::models::FeedType;

pub fn load_more(feed_type: FeedType, next_page: u32, search_query: &str) -> Markup {
    html! {
        div id="load-more-wrapper" class="pt-3 pb-4 text-center" {
            button
                type="button"
                hx-get=(format!("/feed/page?type={}&page={}&query={}", feed_type.as_str(), next_page, search_query))
                hx-target="#load-more-wrapper"
                hx-swap="outerHTML"
                hx-indicator="#load-more-spinner"
                class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-neutral-800/80 hover:bg-neutral-800 text-xs font-semibold text-neutral-300 hover:text-white border border-neutral-700/60 active:scale-95 transition-all shadow-sm"
            {
                span { "Load More Stories" }
                span id="load-more-spinner" class="htmx-indicator animate-spin inline-block w-3.5 h-3.5 border-2 border-amber-500 border-t-transparent rounded-full" {}
            }
        }
    }
}
