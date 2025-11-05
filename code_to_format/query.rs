sqlx::query!(
    r#"
        SELECT   1 AS "one!"
    "#,
    id
 )
.fetch_one(pool)
.await
.map_err(|e| warn!("test query fails, error: {e:?}"))
