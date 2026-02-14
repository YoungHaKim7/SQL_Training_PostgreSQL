use sqlx::Executor;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to default postgres database
    let password = std::env::var("PGPASSWORD").unwrap_or_else(|_| "password".to_string());
    // Connect to default postgres database
    let database_url = format!("postgres://postgres:{}@localhost:5432/postgres", password);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create a new database (PostgreSQL doesn't support IF NOT EXISTS, so we ignore errors)
    let result = pool.execute("CREATE DATABASE rust_demo_db").await;

    match result {
        Ok(_) => println!("Database created successfully!"),
        Err(e) => {
            // Check if error is "database already exists"
            if e.to_string().contains("already exists") {
                println!("Database already exists, continuing...");
            } else {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
