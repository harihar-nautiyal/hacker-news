use maud::{Markup, html};

pub fn empty_stories(search_query: &str) -> Markup {
    html! {
        div class="p-8 text-center text-neutral-500 text-sm" {
            div class="text-2xl mb-2" { "🔍" }
            p { "No stories found." }
            @if !search_query.is_empty() {
                p class="text-xs text-neutral-400 mt-1" { "Try another search keyword." }
            }
        }
    }
}
