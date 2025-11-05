mod common;

#[test]
fn migrate() {
    let content = r###"
{
    sqlx::migrate!(
        r#"alter table test add column log text"#,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| warn!("fails query, error: {e:?}"))
}
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff").unwrap();

    let expected = r###"
{
    sqlx::migrate!(
        r#"
            alter table test add column log text
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| warn!("fails query, error: {e:?}"))
}
    "###;

    common::compare(expected, &formatted);
}

#[test]
fn migrate_no_indent() {
    let content = r###"
sqlx::migrate!(
    r#"alter table test add column log text"#,
)
.fetch_one(pool)
.await
.map_err(|e| warn!("fails query, error: {e:?}"))
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff").unwrap();

    let expected = r###"
sqlx::migrate!(
    r#"
        alter table test add column log text
    "#,
)
.fetch_one(pool)
.await
.map_err(|e| warn!("fails query, error: {e:?}"))
    "###;

    common::compare(expected, &formatted);
}
