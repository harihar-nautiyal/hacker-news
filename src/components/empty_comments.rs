use maud::{Markup, html};

use crate::models::Story;

pub fn empty_comments(detail: &Story) -> Markup {
    html! {
        div class="py-12 text-center text-neutral-500 text-sm" {
            div class="text-2xl mb-2" { "💬" }
            p { "No comments on this story yet." }
            a
                href=(detail.hn_url)
                target="_blank"
                rel="noopener noreferrer"
                class="inline-block mt-3 text-xs text-amber-400 hover:underline"
            {
                "Be the first to join the conversation on Hacker News →"
            }
        }
    }
}
