use crate::models::*;
use crate::utils::feed::format_time_ago;

pub fn build_comment_tree(
    comments: &[AlgoliaComment],
    story_op: &str,
    depth: usize,
) -> (Vec<Comment>, usize) {
    let mut nodes = Vec::new();
    let mut total_count = 0;

    for comment in comments {
        let text = match &comment.text {
            Some(t) if !t.trim().is_empty() => t.clone(),
            _ => {
                if comment.children.is_empty() {
                    continue;
                } else {
                    "[deleted]".to_string()
                }
            }
        };

        total_count += 1;
        let author = comment
            .author
            .clone()
            .unwrap_or_else(|| "[deleted]".to_string());
        let is_op = author == story_op && author != "[deleted]";
        let time_ago = comment
            .created_at_i
            .map(format_time_ago)
            .unwrap_or_else(|| "recently".to_string());

        let (children, children_count) = build_comment_tree(&comment.children, story_op, depth + 1);
        total_count += children_count;

        nodes.push(Comment {
            id: comment.id,
            author,
            text,
            time_ago,
            is_op,
            depth,
            total_replies: children_count,
            children,
        });
    }

    (nodes, total_count)
}
