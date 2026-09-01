use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FeedType {
    Top,
    New,
    Best,
    Ask,
    Show,
    Jobs,
}
