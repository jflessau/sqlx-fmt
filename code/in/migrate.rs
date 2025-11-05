sqlx::migrate!("select 1 from    test")
.fetch_one(pool)
.await
