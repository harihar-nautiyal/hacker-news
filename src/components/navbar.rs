use maud::{Markup, html};

use crate::components::search_bar::search_bar;
use crate::models::FeedType;

pub fn navbar(active_feed: FeedType, search_query: &str) -> Markup {
    html! {
        header class="h-16 border-b border-neutral-800 bg-neutral-900/90 backdrop-blur sticky top-0 z-30 flex-shrink-0 flex items-center justify-between px-4 lg:px-6 gap-4" {
            div class="flex items-center gap-3" {
                a class="flex items-center gap-2.5 group" href="/" hx-get="/feed?type=top" hx-target="#stories-container" hx-push-url="/" hx-preload="mouseover" onclick="setActiveFeed('top')" {
                    div class="w-8 h-8 rounded-lg bg-amber-500 flex items-center justify-center font-bold text-neutral-950 text-lg shadow-md shadow-amber-500/20 group-hover:scale-105 transition-transform" {
                        "Y"
                    }
                    div class="hidden sm:block" {
                        div class="flex items-center gap-1.5 leading-tight" {
                            span class="font-black tracking-tight text-neutral-100 text-base" {
                                "Hacker News"
                            }
                            span class="text-[10px] font-mono font-bold px-1.5 py-0.2 rounded bg-amber-500/20 text-amber-400 border border-amber-500/30" {
                                "SPA"
                            }
                        }
                        p class="text-[10px] text-neutral-400 font-mono" {
                            "HTMX + Maud + Rust"
                        }
                    }
                }
            }

            nav class="hidden md:flex items-center gap-1 bg-neutral-950/80 p-1 rounded-xl border border-neutral-800/80 text-xs font-medium" role="tablist" aria-label="Feeds" {
                @for feed in FeedType::ALL {
                    @let is_active = active_feed == feed;
                    button
                        class=(if is_active {
                            "nav-tab px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5 bg-amber-500 text-neutral-950 font-semibold shadow"
                        } else {
                            "nav-tab px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5 text-neutral-400 hover:text-neutral-200 hover:bg-neutral-800/60"
                        })
                        id=(format!("tab-{}", feed.as_str()))
                        role="tab"
                        aria-selected=(if is_active { "true" } else { "false" })
                        aria-controls="stories-container"
                        data-feed=(feed.as_str())
                        hx-get=(format!("/feed?type={}", feed.as_str()))
                        hx-target="#stories-container"
                        hx-push-url=(format!("/?type={}", feed.as_str()))
                        hx-preload="mouseover"
                        onclick=(format!("setActiveFeed('{}')", feed.as_str()))
                    {
                        span { (feed.icon()) }
                        span { (feed.label()) }
                    }
                }
            }

            div class="flex items-center gap-2 flex-1 md:flex-initial justify-end" {
                (search_bar(search_query))
            }
        }
    }
}
