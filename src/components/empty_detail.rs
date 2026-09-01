use maud::{Markup, html};

pub fn empty_detail() -> Markup {
    html! {
        div class="h-full flex flex-col items-center justify-center p-8 text-center text-neutral-400 select-none" {
            div class="w-16 h-16 rounded-2xl bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-500 mb-4 shadow-lg shadow-amber-500/5" {
                svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z" {}
                }
            }
            h3 class="text-lg font-bold text-neutral-200 mb-1" { "No Story Selected" }
            p class="text-sm text-neutral-400 max-w-sm mb-6" {
                "Click on any story from the list to read the submission, discussion thread, and nested comments."
            }
            div class="grid grid-cols-2 gap-3 max-w-xs text-xs text-left" {
                div class="bg-neutral-900/80 border border-neutral-800 p-3 rounded-xl" {
                    div class="font-semibold text-neutral-300 flex items-center gap-1.5 mb-1" {
                        span class="text-amber-400" { "⚡" } " Fast SPA"
                    }
                    div class="text-neutral-400 text-[11px]" {
                        "HTMX partial swaps with URL push state & instant caching"
                    }
                }
                div class="bg-neutral-900/80 border border-neutral-800 p-3 rounded-xl" {
                    div class="font-semibold text-neutral-300 flex items-center gap-1.5 mb-1" {
                        span class="text-amber-400" { "💬" } " Rich Threads"
                    }
                    div class="text-neutral-400 text-[11px]" {
                        "Collapsible nested comments & OP badge highlights"
                    }
                }
            }
        }
    }
}
