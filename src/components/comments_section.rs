use maud::{Markup, html};

use crate::components::comment_node::comment_node;
use crate::components::empty_comments::empty_comments;
use crate::models::Story;

pub fn comments_section(detail: &Story) -> Markup {
    html! {
        div id="comments-container" class="p-6 md:p-8 flex-1" {
            div class="flex items-center justify-between pb-4 mb-4 border-b border-neutral-800/80" {
                div class="flex items-center gap-2" {
                    h2 class="text-base font-bold text-neutral-200" { "Discussion" }
                    span class="px-2 py-0.5 rounded-full text-xs font-semibold bg-neutral-800 text-neutral-300 border border-neutral-700 font-mono" {
                        (detail.num_comments)
                    }
                }

                @if detail.num_comments > 0 {
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

            // Nested Comments Tree
            @if detail.num_comments > 0 {
                div class="comments-container space-y-3" id="comments-tree" {
                    @for comment in &detail.comments {
                        (comment_node(comment))
                    }
                }
            } @else {
                (empty_comments(detail))
            }
        }
    }
}
