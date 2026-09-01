use maud::{DOCTYPE, Markup, html};
use typed_builder::TypedBuilder;

use crate::components::{
    empty_detail::empty_detail,
    head::head,
    mobile_nav::mobile_nav,
    navbar::navbar,
    scripts::scripts,
    stories_list::stories_list,
    story_detail::story_detail,
};
use crate::models::{FeedType, Story, StorySummary};

#[derive(Debug, Clone, TypedBuilder)]
pub struct Index {
    pub title: String,
    pub active_feed: FeedType,
    pub has_active_detail: bool,
    pub category_icon: String,
    pub category_title: String,
    pub feed_type: FeedType,
    pub search_query: String,
    pub stories: Vec<StorySummary>,
    pub next_page: u32,
    pub active_detail: Option<Story>,
}

impl Index {
    pub fn render(&self) -> Markup {
        let active_story_id = self.active_detail.as_ref().map(|d| d.id);
        let layout_class = if self.has_active_detail {
            "flex-1 flex overflow-hidden relative mobile-view-detail"
        } else {
            "flex-1 flex overflow-hidden relative mobile-view-list"
        };

        html! {
            (DOCTYPE)
            html lang="en" class="h-full bg-neutral-950 text-neutral-100 antialiased dark" {
                head {
                    (head(&self.title))
                }
                body class="h-full flex flex-col overflow-hidden bg-neutral-950 font-sans text-neutral-100 selection:bg-amber-500 selection:text-neutral-950" {
                    div id="global-progress" {}

                    (navbar(self.active_feed, &self.search_query))

                    (mobile_nav(self.active_feed))

                    main id="app-layout" class=(layout_class) {
                        section
                            id="stories-container-col"
                            class="w-full md:w-[420px] lg:w-[480px] xl:w-[520px] md:border-r border-neutral-800/80 flex flex-col h-full bg-neutral-950 flex-shrink-0"
                        {
                            div id="stories-container" class="h-full flex flex-col" {
                                (stories_list(
                                    &self.category_icon,
                                    &self.category_title,
                                    &self.stories,
                                    &self.search_query,
                                    self.feed_type,
                                    self.next_page,
                                ))
                            }
                        }

                        section
                            id="detail-pane-col"
                            class="flex-1 flex flex-col h-full bg-neutral-900/40 relative overflow-hidden"
                        {
                            // Progressive Glowing Top Progress Bar on Item Content Shell
                            div id="detail-progress" class="pointer-events-none absolute top-0 left-0 right-0 h-[3px] z-40 overflow-hidden opacity-0 transition-opacity duration-200" {
                                div id="detail-progress-line" style="width: 0%;" {}
                            }

                            div id="detail-pane" class="h-full flex-1 overflow-hidden" {
                                @if let Some(detail) = &self.active_detail {
                                    (story_detail(detail))
                                } @else {
                                    (empty_detail())
                                }
                            }
                        }
                    }

                    (scripts(active_story_id))
                }
            }
        }
    }
}
