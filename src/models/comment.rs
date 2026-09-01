use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct Comment {
    pub id: i64,
    pub author: String,
    pub text: String,
    pub time_ago: String,
    pub is_op: bool,
    pub depth: bool,
    pub total_replies: usize,
    pub children: Vec<Comment>,
}
