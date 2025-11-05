sqlx::query_scalar!(
    r#"
        select exists(
            select 1 from device
            where id = $1
            and "careHomeID" = $2
        ) as "exists!"
    "#,
    device_id,
    care_home_id
)
.fetch_one(pool)
.await
