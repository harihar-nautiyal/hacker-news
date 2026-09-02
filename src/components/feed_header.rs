use maud::{Markup, html};

use crate::models::{FeedType, StorySummary};

pub fn feed_header(
    category_icon: &str,
    category_title: &str,
    stories: &[StorySummary],
    search_query: &str,
    feed_type: FeedType,
) -> Markup {
    html! {
        div class="px-4 py-3 bg-neutral-900/90 backdrop-blur border-b border-neutral-800 flex items-center justify-between flex-shrink-0" {
            div class="flex items-center gap-2" {
                span class="text-base" aria-hidden="true" { (category_icon) }
                h2 class="text-sm font-bold text-neutral-200" { (category_title) }
                span class="px-2 py-0.5 rounded-full text-[11px] font-mono bg-neutral-800 text-neutral-300 border border-neutral-700/60" {
                    (stories.len()) " stories"
                }
                @if !search_query.is_empty() {
                    span class="px-2 py-0.5 rounded-md text-[11px] bg-amber-500/15 text-amber-400 border border-amber-500/30" {
                        "Query: \"" (search_query) "\""
                    }
                }
            }

            // Quick Refresh Button
            button
                type="button"
                aria-label="Refresh stories"
                hx-get=(format!("/feed?type={}&query={}&refresh=true", feed_type.as_str(), search_query))
                hx-target="#stories-container"
                hx-indicator="#feed-refresh-spinner"
                class="p-1.5 rounded-lg bg-neutral-800/80 hover:bg-neutral-700 text-neutral-300 hover:text-neutral-100 border border-neutral-700/50 transition-all active:scale-95 flex items-center gap-1 text-xs"
                title="Refresh Stories"
            {
                svg id="feed-refresh-icon" xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" {}
                }
                span id="feed-refresh-spinner" class="htmx-indicator animate-spin inline-block w-3.5 h-3.5 border-2 border-amber-500 border-t-transparent rounded-full" aria-hidden="true" {}
            }
        }
    }
}
