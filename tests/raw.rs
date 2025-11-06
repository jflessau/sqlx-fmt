mod common;

#[test]
fn query_as() {
    let content = r###"
    sqlx::query!(
        r#"
            select   *
                from
                    test where id = $1
        "#,
        id
    )
    "###;

    let expected = r###"
    sqlx::query!(
        r#"
            select *
            from
                test
            where id = $1
        "#,
        id
    )
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff").unwrap();
    common::compare(expected, &formatted);
}
