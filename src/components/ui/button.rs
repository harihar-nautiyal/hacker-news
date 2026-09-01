use maud::{Markup, html};
pub fn Button(label: &str) -> Markup {
    html! {
        button class="bg-emerald-500 hover:bg-emerald-600 text-white font-bold py-2 px-4 rounded shadow-md" { (label) }
    }
}
