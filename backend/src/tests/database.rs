use crate::{server::shared::storage::DatabaseMigrations, tests::setup_test_db};
use std::path::Path;

use crate::tests::SERVER_DB_FIXTURE;

#[tokio::test]
async fn test_database_schema_backward_compatibility() {
    let db_path = Path::new(SERVER_DB_FIXTURE);

    if db_path.exists() {
        println!("Testing backward compatibility with database from latest release");

        // Start test database
        let pool = setup_test_db().await;
        
        // Load fixture
        let sql = std::fs::read_to_string(db_path).unwrap();
        sqlx::raw_sql(&sql).execute(&pool).await.unwrap();

        // Try to read from all tables with current code
        let hosts_result = sqlx::query("SELECT * FROM hosts").fetch_all(&pool).await;
        assert!(
            hosts_result.is_ok(),
            "Failed to read hosts table: {:?}",
            hosts_result.err()
        );

        let services_result = sqlx::query("SELECT * FROM services").fetch_all(&pool).await;
        assert!(
            services_result.is_ok(),
            "Failed to read services table: {:?}",
            services_result.err()
        );

        let subnets_result = sqlx::query("SELECT * FROM subnets").fetch_all(&pool).await;
        assert!(
            subnets_result.is_ok(),
            "Failed to read subnets table: {:?}",
            subnets_result.err()
        );

        let groups_result = sqlx::query("SELECT * FROM groups").fetch_all(&pool).await;
        assert!(
            groups_result.is_ok(),
            "Failed to read groups table: {:?}",
            groups_result.err()
        );

        let daemons_result = sqlx::query("SELECT * FROM daemons").fetch_all(&pool).await;
        assert!(
            daemons_result.is_ok(),
            "Failed to read daemons table: {:?}",
            daemons_result.err()
        );

        println!("✅ Successfully read all tables from latest release database");

        // Test that we can apply current schema to the old database
        DatabaseMigrations::initialize(&pool)
            .await
            .expect("Failed to apply current schema to old database");

        println!("✅ Successfully applied current schema to old database");
    } else {
        println!("⚠️  No database fixture found at {}", SERVER_DB_FIXTURE);
        println!("   Run release workflow to generate fixtures");

        assert!(false, "Failed to load database fixture");
    }
}
