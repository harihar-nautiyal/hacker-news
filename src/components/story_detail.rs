use maud::{Markup, html};

use crate::components::comments_section::comments_section;
use crate::components::detail_mobile_bar::detail_mobile_bar;
use crate::components::story_header::story_header;
use crate::models::Story;

/// Full server-rendered story detail view with all comments (used for direct visits and search crawlers)
pub fn story_detail(detail: &Story) -> Markup {
    html! {
        div class="h-full overflow-y-auto bg-neutral-900/60 flex flex-col divide-y divide-neutral-800/80 custom-scrollbar" id="story-detail-content" {
            // Mobile Header / Back Bar
            (detail_mobile_bar(detail))

            // Story Main Header Card
            (story_header(detail))

            // Discussion & Comments Section
            (comments_section(detail))
        }
    }
}

/// Fast lazy-loaded story detail view with instant header and background comments trigger
pub fn story_detail_lazy(detail: &Story) -> Markup {
    html! {
        div class="h-full overflow-y-auto bg-neutral-900/60 flex flex-col divide-y divide-neutral-800/80 custom-scrollbar" id="story-detail-content" {
            // Mobile Header / Back Bar
            (detail_mobile_bar(detail))

            // Story Main Header Card
            (story_header(detail))

            // Discussion & Comments Section
            @if detail.num_comments == 0 {
                (comments_section(detail))
            } @else {
                div
                    id="comments-container"
                    hx-get=(format!("/item/{}/comments", detail.id))
                    hx-trigger="load"
                    hx-swap="outerHTML"
                {
                    (comments_skeleton())
                }
            }
        }
    }
}

/// Comments container placeholder for lazy loading (renders discussion header and leaves body empty until comments load)
pub fn comments_skeleton() -> Markup {
    html! {
        div id="comments-container" class="p-6 md:p-8 flex-1" {
            // Exact same Discussion Header as comments_section
            div class="flex items-center justify-between pb-4 mb-4 border-b border-neutral-800/80" {
                h2 class="text-base font-bold text-neutral-200" { "Discussion" }

                div class="flex items-center gap-2" {
                    button
                        type="button"
                        onclick="toggleAllComments(false)"
                        class="text-xs text-neutral-400 hover:text-neutral-200 px-2.5 py-1 rounded bg-neutral-800/60 hover:bg-neutral-800 border border-neutral-700/50 transition-colors"
                    {
                        "Collapse All"
                    }
                    button
                        type="button"
                        onclick="toggleAllComments(true)"
                        class="text-xs text-neutral-400 hover:text-neutral-200 px-2.5 py-1 rounded bg-neutral-800/60 hover:bg-neutral-800 border border-neutral-700/50 transition-colors"
                    {
                        "Expand All"
                    }
                }
            }
        }
    }
}
