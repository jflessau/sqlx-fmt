mod common;

#[test_log::test]
fn nested() {
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
}
    "###;

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
}
    "###;

    let formatted = sqlx_fmt::format(content, ".sqruff", 4, &None).unwrap();
    common::compare(expected, &formatted);
}
