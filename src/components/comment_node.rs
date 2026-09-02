use maud::{Markup, PreEscaped, html};

use crate::models::Comment;

pub fn comment_node(comment: &Comment) -> Markup {
    let border_class = format!("comment-body mt-1 text-neutral-300 leading-relaxed text-sm pl-3.5 ml-1.5 border-l-2 {} transition-colors", comment.border_class());

    html! {
        div class="comment-thread mt-3 text-sm" id=(format!("comment-{}", comment.id)) {
            details open class="group" {
                summary class="flex items-center gap-2 cursor-pointer select-none text-xs text-neutral-300 py-1 px-1.5 rounded hover:bg-neutral-800/60 transition-colors list-none" aria-label=(format!("Comment thread by {}", comment.author)) {
                    span class="inline-block transition-transform duration-150 transform text-neutral-400 group-open:rotate-90 text-[10px]" aria-hidden="true" { "▶" }
                    span class="font-semibold text-neutral-200 hover:text-amber-400 transition-colors" {
                        (comment.author)
                    }
                    @if comment.is_op {
                        span class="inline-flex items-center px-1.5 py-0.2 rounded text-[10px] font-bold bg-amber-500/20 text-amber-400 border border-amber-500/30 tracking-wider" {
                            "OP"
                        }
                    }
                    span class="text-neutral-500 select-none" aria-hidden="true" { "·" }
                    span class="text-neutral-400 font-mono text-[11px]" { (comment.time_ago) }
                    @if comment.total_replies > 0 {
                        span class="text-[11px] text-neutral-300 ml-auto hidden group-[&:not([open])]:inline-block font-mono bg-neutral-800/80 px-2 py-0.5 rounded-full border border-neutral-700/50" {
                            "+" (comment.total_replies) " replies hidden"
                        }
                    }
                }
                div class=(border_class) {
                    div class="prose prose-invert prose-sm max-w-none text-neutral-300 [&_a]:text-amber-400 [&_a:hover]:underline [&_p]:mb-2 [&_p:last-child]:mb-0 [&_pre]:bg-neutral-950 [&_pre]:p-2.5 [&_pre]:rounded-lg [&_pre]:text-xs [&_pre]:overflow-x-auto [&_code]:text-amber-300 [&_code]:font-mono [&_code]:text-xs" {
                        (PreEscaped(&comment.text))
                    }
                    @if !comment.children.is_empty() {
                        div class="replies" {
                            @for child in &comment.children {
                                (comment_node(child))
                            }
                        }
                    }
                }
            }
        }
    }
}
