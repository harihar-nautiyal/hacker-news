use maud::{Markup, html};

pub fn scripts(active_story_id: Option<i64>) -> Markup {
    html! {
        @if let Some(id) = active_story_id {
            meta name="active-story-id" content=(id);
        }
        script src="/static/scripts/app.js" defer {}
    }
}
