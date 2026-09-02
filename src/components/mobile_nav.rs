use maud::{Markup, html};

use crate::models::FeedType;

pub fn mobile_nav(active_feed: FeedType) -> Markup {
    html! {
        div class="md:hidden flex items-center gap-1 overflow-x-auto px-3 py-2 bg-neutral-900 border-b border-neutral-800 text-xs font-medium custom-scrollbar flex-shrink-0" role="tablist" aria-label="Mobile Feeds" {
            @for feed in FeedType::ALL {
                @let is_active = active_feed == feed;
                button
                    id=(format!("m-tab-{}", feed.as_str()))
                    role="tab"
                    aria-selected=(if is_active { "true" } else { "false" })
                    aria-controls="stories-container"
                    data-feed=(feed.as_str())
                    hx-get=(format!("/feed?type={}", feed.as_str()))
                    hx-target="#stories-container"
                    hx-push-url=(format!("/?type={}", feed.as_str()))
                    hx-preload="mouseover"
                    onclick=(format!("setActiveFeed('{}'); showMobileList();", feed.as_str()))
                    class=(if is_active {
                        "nav-tab flex-shrink-0 px-3 py-1.5 rounded-lg bg-amber-500 text-neutral-950 font-bold"
                    } else {
                        "nav-tab flex-shrink-0 px-3 py-1.5 rounded-lg text-neutral-300 hover:text-white bg-neutral-950 border border-neutral-800"
                    })
                {
                    (feed.icon()) " " (feed.label())
                }
            }
        }
    }
}
