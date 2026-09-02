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
                    (comments_skeleton(detail.num_comments))
                }
            }
        }
    }
}

/// High-fidelity animated comments placeholder skeleton
pub fn comments_skeleton(num_comments: usize) -> Markup {
    html! {
        div id="comments-container" class="p-6 md:p-8 flex-1" {
            div class="flex items-center justify-between pb-4 mb-4 border-b border-neutral-800/80" {
                div class="flex items-center gap-2" {
                    h2 class="text-base font-bold text-neutral-200" { "Discussion" }
                    span class="px-2 py-0.5 rounded-full text-xs font-semibold bg-neutral-800 text-neutral-300 border border-neutral-700 font-mono" {
                        (num_comments)
                    }
                }
                div class="flex items-center gap-2 text-xs text-neutral-400 font-mono" {
                    span class="inline-block w-2 h-2 rounded-full bg-amber-400 animate-ping mr-0.5" {}
                    span class="text-neutral-400" { "Loading comments..." }
                }
            }

            // Skeleton Placeholder Cards
            div class="space-y-3 animate-pulse" {
                @for i in 0..4.min(num_comments.max(1)) {
                    div class="p-3.5 rounded-xl bg-neutral-900/50 border border-neutral-800/60 space-y-2.5" {
                        div class="flex items-center gap-2" {
                            div class="h-3 w-20 bg-neutral-800 rounded" {}
                            div class="h-1.5 w-1.5 bg-neutral-700 rounded-full" {}
                            div class="h-3 w-14 bg-neutral-800/70 rounded" {}
                        }
                        div class="space-y-1.5 pt-0.5" {
                            div class=(if i % 2 == 0 { "h-3.5 w-full bg-neutral-800/60 rounded" } else { "h-3.5 w-11/12 bg-neutral-800/60 rounded" }) {}
                            div class=(if i % 2 == 0 { "h-3.5 w-4/5 bg-neutral-800/40 rounded" } else { "h-3.5 w-2/3 bg-neutral-800/40 rounded" }) {}
                        }
                    }
                }
            }
        }
    }
}
