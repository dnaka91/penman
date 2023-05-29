mod postgres;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
trait SpaceFeedRepo {
    /// Returns a Result of ID or Error
    fn create(&self, name: String) -> Result<String>;
    fn rename(&self, id: String, name: String) -> Result<bool>;
}
