use maud::{Markup, html};
use typed_builder::TypedBuilder;

use crate::components::empty_stories::empty_stories;
use crate::components::feed_header::feed_header;
use crate::components::load_more::load_more;
use crate::components::stories_items::stories_items;
use crate::models::{FeedType, StorySummary};

#[derive(Debug, Clone, TypedBuilder)]
pub struct StoriesList<'a> {
    pub category_icon: &'a str,
    pub category_title: &'a str,
    pub stories: &'a [StorySummary],
    pub search_query: &'a str,
    pub feed_type: FeedType,
    pub next_page: u32,
}

impl<'a> StoriesList<'a> {
    pub fn render(&self) -> Markup {
        html! {
            div class="flex flex-col h-full overflow-hidden" {
                // Feed Header / Filter Bar
                (feed_header(
                    self.category_icon,
                    self.category_title,
                    self.stories,
                    self.search_query,
                    self.feed_type,
                ))

                // Scrollable Story List
                div class="flex-1 overflow-y-auto p-3 space-y-2.5 custom-scrollbar" id="stories-scroll-container" {
                    div id="stories-items" class="space-y-2.5" {
                        (stories_items(self.stories))
                    }

                    @if self.stories.is_empty() {
                        (empty_stories(self.search_query))
                    } @else {
                        (load_more(self.feed_type, self.next_page, self.search_query))
                    }
                }
            }
        }
    }
}

pub fn stories_list(
    category_icon: &str,
    category_title: &str,
    stories: &[StorySummary],
    search_query: &str,
    feed_type: FeedType,
    next_page: u32,
) -> Markup {
    StoriesList::builder()
        .category_icon(category_icon)
        .category_title(category_title)
        .stories(stories)
        .search_query(search_query)
        .feed_type(feed_type)
        .next_page(next_page)
        .build()
        .render()
}
