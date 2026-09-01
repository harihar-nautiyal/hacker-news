use maud::{Markup, html};

use crate::models::Story;

pub fn detail_mobile_bar(detail: &Story) -> Markup {
    html! {
        div class="md:hidden sticky top-0 z-20 flex items-center justify-between px-4 py-3 bg-neutral-900/95 backdrop-blur border-b border-neutral-800" {
            button
                type="button"
                onclick="showMobileList()"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-neutral-800 text-neutral-200 text-xs font-semibold hover:bg-neutral-700 active:scale-95 transition-all"
            {
                svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" {}
                }
                span { "Back to Stories" }
            }

            a
                href=(detail.hn_url)
                target="_blank"
                rel="noopener noreferrer"
                class="text-xs text-amber-400 hover:underline flex items-center gap-1"
            {
                span { "HN Thread" }
                svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" {}
                }
            }
        }
    }
}
