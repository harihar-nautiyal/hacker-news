use actix_web::{
    Responder, get,
    web::{Data, Query},
};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;
use typed_builder::TypedBuilder;

use crate::AppState;
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
use crate::utils::feed::Feed;

#[derive(Debug, Clone, Deserialize)]
pub struct FeedQuery {
    #[serde(rename = "type")]
    pub feed_type: Option<String>,
    pub page: Option<u32>,
    pub query: Option<String>,
    pub refresh: Option<bool>,
}

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

#[get("/")]
pub async fn index(service: Data<AppState>, query: Query<FeedQuery>) -> impl Responder {
    let feed_kind = FeedType::from_str(query.feed_type.as_deref().unwrap_or("top"));
    let stories = service
        .get_feed(feed_kind, 0, query.refresh.unwrap_or(false))
        .await
        .unwrap_or_default();

    Index::builder()
        .title(format!("{} | Hacker News SPA", feed_kind.label()))
        .active_feed(feed_kind)
        .has_active_detail(false)
        .category_icon(feed_kind.icon().to_string())
        .category_title(feed_kind.label().to_string())
        .feed_type(feed_kind)
        .search_query(String::new())
        .stories(stories)
        .next_page(1)
        .active_detail(None)
        .build()
        .render()
}
