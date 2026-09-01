use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct Comment {
    pub id: i64,
    pub author: String,
    pub text: String,
    pub time_ago: String,
    pub is_op: bool,
    pub depth: usize,
    pub total_replies: usize,
    pub children: Vec<Comment>,
}

impl Comment {
    pub fn border_class(&self) -> &'static str {
        match self.depth % 4 {
            0 => "border-neutral-800 hover:border-amber-500/50",
            1 => "border-neutral-800/90 hover:border-blue-500/50",
            2 => "border-neutral-800/80 hover:border-emerald-500/50",
            _ => "border-neutral-800/70 hover:border-purple-500/50",
        }
    }
}
