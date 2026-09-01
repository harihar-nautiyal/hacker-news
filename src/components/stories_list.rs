use maud::{Markup, html};

use crate::components::empty_stories::empty_stories;
use crate::components::feed_header::feed_header;
use crate::components::load_more::load_more;
use crate::components::stories_items::stories_items;
use crate::models::{FeedType, StorySummary};

pub fn stories_list(
    category_icon: &str,
    category_title: &str,
    stories: &[StorySummary],
    search_query: &str,
    feed_type: FeedType,
    next_page: u32,
) -> Markup {
    html! {
        div class="flex flex-col h-full overflow-hidden" {
            // Feed Header / Filter Bar
            (feed_header(category_icon, category_title, stories, search_query, feed_type))

            // Scrollable Story List
            div class="flex-1 overflow-y-auto p-3 space-y-2.5 custom-scrollbar" id="stories-scroll-container" {
                div id="stories-items" class="space-y-2.5" {
                    (stories_items(stories))
                }

                @if stories.is_empty() {
                    (empty_stories(search_query))
                } @else {
                    (load_more(feed_type, next_page, search_query))
                }
            }
        }
    }
}
