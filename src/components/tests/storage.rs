use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::shared::storage::{StorageResult,StorageError};
use super::types::{Test,Layer};

#[async_trait]
pub trait TestStorage: Send + Sync {
    async fn get_tests(&self) -> StorageResult<Vec<Test>>;
    async fn get_test(&self, id: &str) -> StorageResult<Test>;
    async fn save_test(&self, test: &Test) -> StorageResult<()>;
    async fn update_test(&self, id: &str, test: &Test) -> StorageResult<()>;
    async fn delete_test(&self, id: &str) -> StorageResult<()>;
}

pub struct SqliteTestStorage {
    pool: SqlitePool,
}

impl SqliteTestStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl TestStorage for SqliteTestStorage {
    async fn get_tests(&self) -> StorageResult<Vec<Test>> {
        let rows = sqlx::query!(
            "SELECT id, name, description, layers, created_at, updated_at FROM tests ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let tests = rows
            .into_iter()
            .map(|row| -> StorageResult<Test> {
                let layers: Vec<Layer> = serde_json::from_str(&row.layers)?;
                Ok(Test {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    layers,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                        .unwrap()
                        .with_timezone(&chrono::Utc),
                })
            })
            .collect::<StorageResult<Vec<_>>>()?;

        Ok(tests)
    }

    async fn get_test(&self, id: &str) -> StorageResult<Test> {
        let row = sqlx::query!(
            "SELECT id, name, description, layers, created_at, updated_at FROM tests WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(StorageError::NotFound)?;

        let layers: Vec<Layer> = serde_json::from_str(&row.layers)?;
        
        Ok(Test {
            id: row.id,
            name: row.name,
            description: row.description,
            layers,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        })
    }

    async fn save_test(&self, test: &Test) -> StorageResult<()> {
        let layers = serde_json::to_string(&test.layers)?;
        let created_at_str = test.created_at.to_rfc3339();
        let updated_at_str = test.updated_at.to_rfc3339();
        
        sqlx::query!(
            "INSERT INTO tests (id, name, description, layers, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
            test.id,
            test.name,
            test.description,
            layers,
            created_at_str,
            updated_at_str
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_test(&self, id: &str, test: &Test) -> StorageResult<()> {
        let layers = serde_json::to_string(&test.layers)?;
        let updated_at_str = test.updated_at.to_rfc3339();
        
        let result = sqlx::query!(
            "UPDATE tests SET name = ?, description = ?, layers = ?, updated_at = ? WHERE id = ?",
            test.name,
            test.description,
            layers,
            updated_at_str,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }

    async fn delete_test(&self, id: &str) -> StorageResult<()> {
        let result = sqlx::query!("DELETE FROM tests WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }

        Ok(())
    }
}