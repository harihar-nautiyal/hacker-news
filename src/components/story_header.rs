use maud::{Markup, PreEscaped, html};

use crate::models::Story;

pub fn story_header(detail: &Story) -> Markup {
    html! {
        div class="p-6 md:p-8 bg-gradient-to-b from-neutral-900 to-neutral-900/40" {
            div class="flex flex-wrap items-center gap-2 mb-3" {
                span class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-bold bg-amber-500/10 text-amber-400 border border-amber-500/20 font-mono" {
                    svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M5 15l7-7 7 7" {}
                    }
                    (detail.points) " pts"
                }

                @if let Some(domain) = &detail.domain {
                    span class="px-2.5 py-1 rounded-full text-xs font-medium bg-neutral-800 text-neutral-300 border border-neutral-700/60 font-mono flex items-center gap-1" {
                        svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 text-neutral-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" {}
                        }
                        (domain)
                    }
                }

                span class="text-xs text-neutral-400" {
                    "submitted " (detail.time_ago) " by "
                    span class="font-medium text-neutral-200" {
                        "@" (detail.author)
                    }
                }
            }

            h1 class="text-xl md:text-2xl font-bold text-neutral-100 leading-snug tracking-tight mb-4" {
                @if let Some(url) = &detail.url {
                    a
                        href=(url)
                        target="_blank"
                        rel="noopener noreferrer"
                        class="hover:text-amber-400 transition-colors inline-flex items-start gap-1.5 group"
                    {
                        span { (detail.title) }
                        svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-neutral-400 group-hover:text-amber-400 mt-1 flex-shrink-0 transition-transform group-hover:translate-x-0.5 group-hover:-translate-y-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" {}
                        }
                    }
                } @else {
                    (detail.title)
                }
            }

            (story_actions(detail))

            (story_text(detail))
        }
    }
}

pub fn story_actions(detail: &Story) -> Markup {
    html! {
        div class="flex flex-wrap items-center gap-2.5 pt-2" {
            @if let Some(url) = &detail.url {
                a
                    href=(url)
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-semibold bg-amber-500 hover:bg-amber-400 text-neutral-950 shadow-md shadow-amber-500/20 active:scale-95 transition-all"
                {
                    span { "Open Article" }
                    svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" {}
                    }
                }
            }

            a
                href=(detail.hn_url)
                target="_blank"
                rel="noopener noreferrer"
                class="inline-flex items-center gap-1.5 px-3.5 py-2 rounded-xl text-xs font-medium bg-neutral-800/80 hover:bg-neutral-700 text-neutral-200 border border-neutral-700/60 active:scale-95 transition-all"
                title="View discussion thread on Hacker News"
            {
                svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-amber-500" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" {
                    path d="M0 0v24h24V0H0zm12.8 13.7v5.5H11.2v-5.5L7.1 5.8h2.3l2.6 5.4 2.6-5.4h2.3l-4.1 7.9z" {}
                }
                span { "View on HN" }
            }

            button
                type="button"
                aria-label="Refresh discussion"
                hx-get=(format!("/item/{}?refresh=true", detail.id))
                hx-target="#detail-pane"
                hx-indicator="#refresh-item-spinner"
                class="inline-flex items-center gap-1.5 px-3.5 py-2 rounded-xl text-xs font-medium bg-neutral-800/80 hover:bg-neutral-700 text-neutral-200 border border-neutral-700/60 active:scale-95 transition-all ml-auto"
            {
                svg id="refresh-item-icon" xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-neutral-400 group-hover:rotate-180 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" {}
                }
                span id="refresh-item-spinner" class="htmx-indicator animate-spin inline-block w-3.5 h-3.5 border-2 border-amber-500 border-t-transparent rounded-full" aria-hidden="true" {}
                span { "Refresh Discussion" }
            }
        }
    }
}

pub fn story_text(detail: &Story) -> Markup {
    html! {
        @if let Some(text) = &detail.text {
            @if !text.trim().is_empty() {
                div class="mt-6 p-5 rounded-2xl bg-neutral-950/70 border border-neutral-800/90 text-neutral-200 text-sm leading-relaxed prose prose-invert max-w-none [&_a]:text-amber-400 [&_a:hover]:underline [&_p]:mb-3 [&_p:last-child]:mb-0 [&_pre]:bg-neutral-900 [&_pre]:p-3 [&_pre]:rounded-xl [&_pre]:text-xs [&_pre]:overflow-x-auto [&_code]:text-amber-300 [&_code]:font-mono [&_code]:text-xs shadow-inner" {
                    (PreEscaped(text))
                }
            }
        }
    }
}
