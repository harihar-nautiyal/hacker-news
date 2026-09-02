use maud::{Markup, html};

use crate::models::StorySummary;

pub fn story_card(story: &StorySummary) -> Markup {
    html! {
        article
            id=(format!("story-card-{}", story.id))
            class="story-card group relative p-4 rounded-xl bg-neutral-900/80 hover:bg-neutral-800/80 border border-neutral-800/80 hover:border-neutral-700/80 cursor-pointer transition-all duration-150 select-none shadow-sm hover:shadow"
            hx-get=(format!("/item/{}", story.id))
            hx-target="#detail-pane"
            hx-push-url=(format!("/item/{}", story.id))
            hx-swap="innerHTML"
            hx-indicator="#detail-progress"
            hx-preload="mouseover"
            onclick=(format!("selectStoryCard('{}'); showMobileDetail();", story.id))
        {
            div class="flex items-start gap-3" {
                // Rank Number
                span class="text-xs font-mono font-semibold text-neutral-300 w-5 pt-0.5 text-right flex-shrink-0 group-hover:text-amber-500 transition-colors" {
                    (story.rank)
                }

                // Main Content
                div class="flex-1 min-w-0" {
                    // Story Title & Domain
                    div class="flex flex-wrap items-baseline gap-x-2 gap-y-1 mb-1.5" {
                        h2 class="text-sm font-medium text-neutral-100 group-hover:text-amber-400 transition-colors leading-snug break-words" {
                            (story.title)
                        }

                        @if let Some(domain) = &story.domain {
                            span class="inline-flex items-center text-[11px] font-mono text-neutral-300 hover:text-neutral-100 transition-colors" onclick="event.stopPropagation()" {
                                @if let Some(url) = &story.url {
                                    a href=(url) target="_blank" rel="noopener noreferrer" class="hover:underline flex items-center gap-0.5" title="Open external link" aria-label=(format!("Open external link to {}", domain)) {
                                        span { "(" (domain) ")" }
                                        svg xmlns="http://www.w3.org/2000/svg" class="w-2.5 h-2.5 opacity-60" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" {}
                                        }
                                    }
                                } @else {
                                    "(" (domain) ")"
                                }
                            }
                        }
                    }

                    // Metadata Pills
                    div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-neutral-300" {
                        // Points
                        span class="inline-flex items-center gap-1 font-mono font-semibold text-amber-400 text-[11px]" {
                            svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 15l7-7 7 7" {}
                            }
                            (story.points)
                        }

                        span class="text-neutral-500 select-none" aria-hidden="true" { "·" }

                        // Author
                        span class="truncate hover:text-neutral-100" {
                            "by @" (story.author)
                        }

                        span class="text-neutral-500 select-none" aria-hidden="true" { "·" }

                        // Time
                        span class="font-mono text-[11px] text-neutral-300" {
                            (story.time_ago)
                        }

                        span class="text-neutral-500 select-none" aria-hidden="true" { "·" }

                        // Comments Count
                        a
                            href=(story.hn_url())
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center gap-1 hover:text-neutral-100 font-mono text-[11px] text-neutral-300"
                            onclick="event.stopPropagation()"
                            title="View discussion on Hacker News"
                            aria-label=(format!("{} comments on Hacker News", story.num_comments))
                        {
                            svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 text-neutral-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" {}
                            }
                            (story.num_comments)
                        }
                    }
                }
            }
        }
    }
}
