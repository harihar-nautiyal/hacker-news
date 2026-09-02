use crate::models::{FeedType, Story, StorySummary};
use anyhow::{Context, Result};
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const TABLE_STORIES: TableDefinition<u64, &[u8]> = TableDefinition::new("stories");
const TABLE_FEEDS: TableDefinition<&str, &[u8]> = TableDefinition::new("feeds");
const TABLE_SEARCH: TableDefinition<&str, &[u8]> = TableDefinition::new("search");
const TABLE_META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");

#[derive(Serialize, Deserialize)]
struct CachedFeedEntry {
    stories: Vec<StorySummary>,
    saved_at_epoch_secs: u64,
}

#[derive(Clone)]
pub struct DbStore {
    db: Arc<Database>,
}

fn current_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl DbStore {
    /// Opens or creates the redb database at the specified path and initializes tables.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Database::create(path).context("Failed to open or create redb database")?;

        // Initialize all tables in an initial write transaction
        let write_txn = db.begin_write().context("Failed to begin init write txn")?;
        {
            let _ = write_txn.open_table(TABLE_STORIES)?;
            let _ = write_txn.open_table(TABLE_FEEDS)?;
            let _ = write_txn.open_table(TABLE_SEARCH)?;
            let _ = write_txn.open_table(TABLE_META)?;
        }
        write_txn.commit().context("Failed to commit init write txn")?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Retrieve a single story detail and comments by its ID.
    pub fn get_story(&self, id: i64) -> Result<Option<Story>> {
        let read_txn = self.db.begin_read().context("Failed to begin read txn")?;
        let table = match read_txn.open_table(TABLE_STORIES) {
            Ok(t) => t,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        if let Some(guard) = table.get(id as u64)? {
            let story: Story = bincode::deserialize(guard.value())
                .context("Failed to deserialize story from redb")?;
            return Ok(Some(story));
        }

        Ok(None)
    }

    /// Persist a story detail and its comments to the database.
    pub fn put_story(&self, story: &Story) -> Result<()> {
        let bytes = bincode::serialize(story).context("Failed to serialize story")?;
        let write_txn = self.db.begin_write().context("Failed to begin write txn")?;
        {
            let mut table = write_txn.open_table(TABLE_STORIES)?;
            table.insert(story.id as u64, bytes.as_slice())?;
        }
        write_txn.commit().context("Failed to commit write txn")?;
        Ok(())
    }

    /// Batch insert stories in a single ACID transaction for high write performance.
    pub fn put_stories_batch(&self, stories: &[Story]) -> Result<()> {
        if stories.is_empty() {
            return Ok(());
        }

        let write_txn = self.db.begin_write().context("Failed to begin batch write txn")?;
        {
            let mut table = write_txn.open_table(TABLE_STORIES)?;
            for story in stories {
                let bytes = bincode::serialize(story).context("Failed to serialize story in batch")?;
                table.insert(story.id as u64, bytes.as_slice())?;
            }
        }
        write_txn.commit().context("Failed to commit batch write txn")?;
        Ok(())
    }

    /// Retrieve a cached feed list if it exists and has not exceeded `max_age_secs`.
    pub fn get_feed(
        &self,
        feed_type: FeedType,
        page: u32,
        max_age_secs: Option<u64>,
    ) -> Result<Option<Vec<StorySummary>>> {
        let key = format!("{}:{}", feed_type.as_str(), page);
        let read_txn = self.db.begin_read().context("Failed to begin read txn")?;
        let table = match read_txn.open_table(TABLE_FEEDS) {
            Ok(t) => t,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        if let Some(guard) = table.get(key.as_str())? {
            let entry: CachedFeedEntry = bincode::deserialize(guard.value())
                .context("Failed to deserialize feed from redb")?;

            if let Some(max_age) = max_age_secs {
                let now = current_epoch_secs();
                if max_age == 0 || now.saturating_sub(entry.saved_at_epoch_secs) >= max_age {
                    return Ok(None); // Expired
                }
            }

            return Ok(Some(entry.stories));
        }

        Ok(None)
    }

    /// Persist a feed list to the database with a timestamp.
    pub fn put_feed(&self, feed_type: FeedType, page: u32, stories: &[StorySummary]) -> Result<()> {
        let key = format!("{}:{}", feed_type.as_str(), page);
        let entry = CachedFeedEntry {
            stories: stories.to_vec(),
            saved_at_epoch_secs: current_epoch_secs(),
        };
        let bytes = bincode::serialize(&entry).context("Failed to serialize feed entry")?;

        let write_txn = self.db.begin_write().context("Failed to begin write txn")?;
        {
            let mut table = write_txn.open_table(TABLE_FEEDS)?;
            table.insert(key.as_str(), bytes.as_slice())?;
        }
        write_txn.commit().context("Failed to commit write txn")?;
        Ok(())
    }

    /// Retrieve cached search results if valid.
    pub fn get_search(
        &self,
        query: &str,
        page: u32,
        max_age_secs: Option<u64>,
    ) -> Result<Option<Vec<StorySummary>>> {
        let key = format!("{}:{}", query.to_lowercase().trim(), page);
        let read_txn = self.db.begin_read().context("Failed to begin read txn")?;
        let table = match read_txn.open_table(TABLE_SEARCH) {
            Ok(t) => t,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        if let Some(guard) = table.get(key.as_str())? {
            let entry: CachedFeedEntry = bincode::deserialize(guard.value())
                .context("Failed to deserialize search entry from redb")?;

            if let Some(max_age) = max_age_secs {
                let now = current_epoch_secs();
                if max_age == 0 || now.saturating_sub(entry.saved_at_epoch_secs) >= max_age {
                    return Ok(None); // Expired
                }
            }

            return Ok(Some(entry.stories));
        }

        Ok(None)
    }

    /// Persist search results to the database with a timestamp.
    pub fn put_search(&self, query: &str, page: u32, stories: &[StorySummary]) -> Result<()> {
        let key = format!("{}:{}", query.to_lowercase().trim(), page);
        let entry = CachedFeedEntry {
            stories: stories.to_vec(),
            saved_at_epoch_secs: current_epoch_secs(),
        };
        let bytes = bincode::serialize(&entry).context("Failed to serialize search entry")?;

        let write_txn = self.db.begin_write().context("Failed to begin write txn")?;
        {
            let mut table = write_txn.open_table(TABLE_SEARCH)?;
            table.insert(key.as_str(), bytes.as_slice())?;
        }
        write_txn.commit().context("Failed to commit write txn")?;
        Ok(())
    }

    /// Count total stories currently persisted in the database.
    pub fn count_stories(&self) -> Result<usize> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE_STORIES)?;
        let mut count = 0;
        let mut iter = table.iter()?;
        while let Some(_) = iter.next() {
            count += 1;
        }
        Ok(count)
    }

    /// Count total cached feed pages in the database.
    pub fn count_feeds(&self) -> Result<usize> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE_FEEDS)?;
        let mut count = 0;
        let mut iter = table.iter()?;
        while let Some(_) = iter.next() {
            count += 1;
        }
        Ok(count)
    }

    /// Count total cached search query pages in the database.
    pub fn count_searches(&self) -> Result<usize> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE_SEARCH)?;
        let mut count = 0;
        let mut iter = table.iter()?;
        while let Some(_) = iter.next() {
            count += 1;
        }
        Ok(count)
    }
}
