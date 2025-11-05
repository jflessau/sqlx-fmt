mod common;

#[test]
fn query() {
    let content = r###"
{
    sqlx::query!(
        r#"
            select   *
                from
                    test where id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| warn!("fails query, error: {e:?}"))
}
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff").unwrap();

    let expected = r###"
{
    sqlx::query!(
        r#"
            select *
            from
                test
            where id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| warn!("fails query, error: {e:?}"))
}
    "###;

    common::compare(expected, &formatted);
}
