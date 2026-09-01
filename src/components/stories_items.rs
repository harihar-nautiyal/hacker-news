use maud::{Markup, html};

use crate::components::story_card::story_card;
use crate::models::StorySummary;

pub fn stories_items(stories: &[StorySummary]) -> Markup {
    html! {
        @for story in stories {
            (story_card(story))
        }
    }
}
