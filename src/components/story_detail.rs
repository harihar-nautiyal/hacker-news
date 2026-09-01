use maud::{Markup, html};

use crate::components::comments_section::comments_section;
use crate::components::detail_mobile_bar::detail_mobile_bar;
use crate::components::story_header::story_header;
use crate::models::Story;

pub fn story_detail(detail: &Story) -> Markup {
    html! {
        div class="h-full overflow-y-auto bg-neutral-900/60 flex flex-col divide-y divide-neutral-800/80 custom-scrollbar" id="story-detail-content" {
            // Mobile Header / Back Bar
            (detail_mobile_bar(detail))

            // Story Main Header Card
            (story_header(detail))

            // Discussion & Comments Section
            (comments_section(detail))
        }
    }
}
