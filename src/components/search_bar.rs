use maud::{Markup, html};

pub fn search_bar(search_query: &str) -> Markup {
    html! {
        div class="relative w-full max-w-xs" {
            div class="absolute left-2.5 top-1/2 -translate-y-1/2 text-neutral-400 pointer-events-none" {
                svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" {}
                }
            }
            input
                id="search-input"
                type="search"
                name="q"
                value=(search_query)
                placeholder="Search stories... (/)"
                aria-label="Search stories"
                autocomplete="off"
                hx-get="/search"
                hx-trigger="input changed delay:300ms, search"
                hx-target="#stories-container"
                hx-indicator="#search-spinner"
                hx-sync="this:replace"
                class="w-full bg-neutral-950/90 border border-neutral-800 focus:border-amber-500/80 focus:ring-1 focus:ring-amber-500/80 rounded-xl pl-9 pr-8 py-1.5 text-xs text-neutral-200 placeholder-neutral-400 outline-none transition-all";

            div id="search-spinner" class="htmx-indicator absolute right-2.5 top-1/2 -translate-y-1/2" {
                span class="animate-spin inline-block w-3.5 h-3.5 border-2 border-amber-500 border-t-transparent rounded-full" aria-hidden="true" {}
            }
        }
    }
}
