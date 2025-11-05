sqlx::query_scalar!(
    Device,
r#"
        select 2
        from
            tesdt
"#,
    device_id,
    care_home_id
)
.fetch_one(pool)
.await
