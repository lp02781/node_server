use crate::json;
const ALLOWED_TABLES: &[&str] = &["websocket", "tcp", "sm_cpp", "sm_rust", "mqtt"];

pub async fn send_database(table_name: &str, payload: json::NodePayload, db_pool: &sqlx::PgPool,) -> Result<(), sqlx::Error> {
    if !ALLOWED_TABLES.contains(&table_name) {
        return Err(sqlx::Error::RowNotFound); 
    }

    let query = format!(
        "INSERT INTO {} (timestamp, temperature, humidity, current) VALUES ($1, $2, $3, $4)", table_name
    );

    sqlx::query(&query)
        .bind(payload.timestamp)
        .bind(payload.data.temperature)
        .bind(payload.data.humidity as i32)
        .bind(payload.data.current)
        .execute(db_pool)
        .await?;

    Ok(())
}

pub async fn prune_old_rows(table_name: &str, db_pool: &sqlx::PgPool,) -> Result<(), sqlx::Error> {
    if !ALLOWED_TABLES.contains(&table_name) {
        return Err(sqlx::Error::RowNotFound); 
    }

    let delete_query = format!(
        r#"
        DELETE FROM {t}
        WHERE ctid IN (
            SELECT ctid FROM {t}
            ORDER BY timestamp ASC
            LIMIT (
                SELECT GREATEST(count(*) - 10, 0) FROM {t}
            )
        )
        "#,
        t = table_name
    );

    sqlx::query(&delete_query)
        .execute(db_pool)
        .await?;

    Ok(())
}