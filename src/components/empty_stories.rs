use maud::{Markup, html};

pub fn empty_stories(search_query: &str) -> Markup {
    html! {
        div class="p-8 text-center text-neutral-400 text-sm" {
            div class="text-2xl mb-2" aria-hidden="true" { "🔍" }
            p { "No stories found." }
            @if !search_query.is_empty() {
                p class="text-xs text-neutral-300 mt-1" { "Try another search keyword." }
            }
        }
    }
}
